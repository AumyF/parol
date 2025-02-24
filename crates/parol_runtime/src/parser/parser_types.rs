use crate::{
    FileSource, FormatToken, Location, LookaheadDFA, NonTerminalIndex, ParseStack, ParseTreeType,
    ParseType, ParserError, ProductionIndex, Result, TokenStream, TokenVec, UnexpectedToken,
    UserActionsTrait,
};
use log::{debug, trace};
use std::cell::RefCell;
use syntree::{Builder, Tree};

///
/// The type that contains all data to process a production within the parser.
///
#[derive(Debug, Clone)]
pub struct Production {
    ///
    /// The non-terminal index of the symbol on the left-hand side of the
    /// production.
    /// It is used as index into the generated LOOKAHEAD_AUTOMATA array.
    ///
    pub lhs: NonTerminalIndex,

    ///
    /// The right-hand side of the production in *reversed order*.
    /// Is pushed onto the parse stack when a production has been chosen for
    /// parsing.
    ///
    pub production: &'static [ParseType],
}

impl Production {
    fn to_string(
        &self,
        terminal_names: &'static [&'static str],
        non_terminal_names: &'static [&'static str],
    ) -> String {
        let rhs = self
            .production
            .iter()
            .rev()
            .map(|s| match s {
                ParseType::N(n) => non_terminal_names[*n].to_owned(),
                ParseType::T(t) => format!(r#""{}""#, terminal_names[*t]),
                ParseType::S(s) => format!("%sc({})", s),
                ParseType::Push(s) => format!("%push({})", s),
                ParseType::Pop => "%pop".to_string(),
                _ => "?".to_owned(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        format!("{}: {};", non_terminal_names[self.lhs], rhs)
    }
}

/// The type used for the parse tree
pub type ParseTree<'t> = Tree<ParseTreeType<'t>, u32, usize>;

/// The parse tree builder type
type TreeBuilder<'t> = Builder<ParseTreeType<'t>, u32, usize>;

///
/// The actual LLK parser.
/// It resembles a PDA.
/// All data of the generated parser are provided in the 'new' function.
///
/// The lifetime parameter `'t` refers to the lifetime of the scanned text.
///
#[derive(Debug)]
pub struct LLKParser<'t> {
    ///
    /// The non-terminal index of the start symbol
    ///
    start_symbol_index: NonTerminalIndex,

    ///
    /// Grammar productions stack; is built up in push_production and reduced after
    /// each processed token/variable
    ///
    parser_stack: ParseStack,

    ///
    /// The production depth. Use for logging reasons only.
    ///
    pub production_depth: usize,

    ///
    /// Temporary stack that receives recognized grammar symbols before they
    /// are added to the parse tree.
    /// This stack is also used to provide arguments to semantic user actions.
    ///
    parse_tree_stack: Vec<ParseTreeType<'t>>,

    ///
    /// The array of generated lookahead automata.
    ///
    lookahead_automata: &'static [LookaheadDFA],

    ///
    /// The array of generated grammar productions.
    ///
    productions: &'static [Production],

    ///
    /// Array of generated terminal names.
    ///
    terminal_names: &'static [&'static str],

    ///
    /// Array of generated non-terminal names.
    ///
    non_terminal_names: &'static [&'static str],

    ///
    /// Enables trimming of the parse tree during parsing.
    /// Thus the parse tree doesn't grow much and runtime overhead is diminished.
    /// Useful when enabling production mode and the whole parse tree is not needed.
    ///
    /// To enable this call the method `trim_parse_tree` on the parser object before parsing.
    ///
    trim_parse_tree: bool,
}

impl<'t> LLKParser<'t> {
    ///
    /// Creates a new instance with the given parameters.
    ///
    pub fn new(
        start_symbol_index: NonTerminalIndex,
        lookahead_automata: &'static [LookaheadDFA],
        productions: &'static [Production],
        terminal_names: &'static [&'static str],
        non_terminal_names: &'static [&'static str],
    ) -> Self {
        Self {
            start_symbol_index,
            parser_stack: ParseStack::new(terminal_names, non_terminal_names),
            production_depth: 0,
            parse_tree_stack: Vec::new(),
            lookahead_automata,
            productions,
            terminal_names,
            non_terminal_names,
            trim_parse_tree: false,
        }
    }

    ///
    /// Call this method to enable trimming of the parse tree during parsing before parsing.
    ///
    /// Doing so the parse tree doesn't grow much and runtime overhead is diminished.
    /// Useful when enabling production mode and the whole parse tree is not needed.
    ///
    pub fn trim_parse_tree(&mut self) {
        self.trim_parse_tree = true;
    }

    fn input_accepted(&self) -> bool {
        matches!(self.parser_stack.stack[..], [] | [ParseType::T(0)])
    }

    fn current_production(&self) -> Option<ProductionIndex> {
        for e in self.parser_stack.stack.iter().rev() {
            if let ParseType::E(p) = e {
                return Some(*p);
            }
        }
        None
    }

    fn push_production(
        &mut self,
        tree_builder: &mut TreeBuilder<'t>,
        prod_num: ProductionIndex,
    ) -> Result<()> {
        self.parser_stack.stack.push(ParseType::E(prod_num));
        for s in self.productions[prod_num].production {
            self.parser_stack.stack.push(*s);
        }

        // Open a 'production entry' node in the tree
        if !self.trim_parse_tree {
            tree_builder
                .open(ParseTreeType::N(
                    self.non_terminal_names[self.productions[prod_num].lhs],
                ))
                .map_err(|source| ParserError::TreeError { source })?;
        }

        // Now push a 'production entry' onto the parse stack
        self.parse_tree_stack.push(ParseTreeType::N(
            self.non_terminal_names[self.productions[prod_num].lhs],
        ));

        self.production_depth += 1;
        trace!(
            "Pushed production {}({}) -> depth {}",
            prod_num,
            self.productions[prod_num].production.len(),
            self.production_depth
        );

        Ok(())
    }

    fn process_item_stack<'u>(
        &mut self,
        tree_builder: &mut TreeBuilder<'t>,
        prod_num: ProductionIndex,
        user_actions: &'u mut dyn UserActionsTrait<'t>,
    ) -> Result<()> {
        let l = self.productions[prod_num]
            .production
            .iter()
            .filter(|s| !s.is_switch())
            .count();
        // We remove the last n entries from the parse tree stack and insert them as
        // children under the node laying below on the stack
        let children = self
            .parse_tree_stack
            .split_off(self.parse_tree_stack.len() - l);

        // With the children we can call the user's semantic action
        user_actions.call_semantic_action_for_production_number(prod_num, &children)?;

        if !self.trim_parse_tree {
            // And we close the production subtree
            tree_builder
                .close()
                .map_err(|source| ParserError::TreeError { source }.into())
        } else {
            Ok(())
        }
    }

    fn predict_production(
        &mut self,
        non_terminal: NonTerminalIndex,
        stream: &RefCell<TokenStream<'t>>,
    ) -> Result<ProductionIndex> {
        let lookahead_dfa = &self.lookahead_automata[non_terminal];
        Ok(lookahead_dfa.eval(&mut stream.borrow_mut())?)
    }

    fn handle_comments<'u>(
        &mut self,
        stream: &RefCell<TokenStream<'t>>,
        user_actions: &'u mut dyn UserActionsTrait<'t>,
    ) -> Result<()> {
        stream
            .borrow_mut()
            .drain_comments()
            .into_iter()
            .for_each(|c| user_actions.on_comment_parsed(c));
        Ok(())
    }

    fn diagnostic_message(&self, msg: &str) -> String {
        trace!("\nParser stack:\n{}\n", self.parser_stack);
        if let Some(prod_num) = self.current_production() {
            format!(
                "{}\n\
                Current production is:\n\
                /* {} */ {}\n",
                msg,
                prod_num,
                self.productions[prod_num].to_string(self.terminal_names, self.non_terminal_names),
            )
        } else {
            format!("{}\n", msg,)
        }
    }

    ///
    /// The actual parsing function.
    /// It is normally not called directly.
    /// The generated parser sources contain all appropriate initialization and
    /// the actual execution of this parse function.
    ///
    pub fn parse<'u>(
        &mut self,
        stream: RefCell<TokenStream<'t>>,
        user_actions: &'u mut dyn UserActionsTrait<'t>,
    ) -> Result<ParseTree<'t>> {
        let prod_num = match self.predict_production(self.start_symbol_index, &stream) {
            Ok(prod_num) => prod_num,
            Err(source) => {
                let nt_name = self.non_terminal_names[self.start_symbol_index];
                let (message, unexpected_tokens, expected_tokens) = self.lookahead_automata
                    [self.start_symbol_index]
                    .build_error(self.terminal_names, &stream.borrow())?;
                return Err(ParserError::PredictionErrorWithExpectations {
                    cause: self.diagnostic_message(
                        format!(
                            "{}\nat non-terminal \"{}\" \n\
                                Current scanner is {}",
                            message,
                            nt_name,
                            &stream.borrow().current_scanner(),
                        )
                        .as_str(),
                    ),
                    input: Box::new(FileSource::from_stream(&stream.borrow())),
                    error_location: unexpected_tokens
                        .get(0)
                        .map_or(Box::<Location>::default(), |t| Box::new(t.token.clone())),
                    unexpected_tokens,
                    expected_tokens,
                    source: Some(Box::new(source)),
                }
                .into());
            }
        };

        let mut tree_builder = TreeBuilder::new();

        self.push_production(&mut tree_builder, prod_num)?;

        while !self.input_accepted() {
            if let Some(entry) = self.parser_stack.stack.last().cloned() {
                match entry {
                    ParseType::T(t) => {
                        let token = stream.borrow_mut().lookahead(0)?;
                        if token.token_type == t {
                            trace!("Consuming token {}", token);
                            self.handle_comments(&stream, user_actions)?;
                            stream.borrow_mut().consume()?;
                            self.parser_stack.stack.pop();
                            if !self.trim_parse_tree {
                                tree_builder
                                    .token(ParseTreeType::T(token.clone()), 1)
                                    .map_err(|source| ParserError::TreeError { source })?;
                            }
                            self.parse_tree_stack.push(ParseTreeType::T(token));
                        } else {
                            let mut expected_tokens = TokenVec::default();
                            expected_tokens.push(self.terminal_names[t].to_string());
                            return Err(ParserError::PredictionErrorWithExpectations {
                                cause: self.diagnostic_message(
                                    format!(
                                        "Found \"{}\" \n\
                                        Current scanner is {}",
                                        token.format(self.terminal_names),
                                        stream.borrow().current_scanner(),
                                    )
                                    .as_str(),
                                ),
                                input: Box::new(FileSource::from_stream(&stream.borrow())),
                                error_location: Box::new((&token).into()),
                                unexpected_tokens: vec![UnexpectedToken::new(
                                    "LA(1)".to_owned(),
                                    self.terminal_names[token.token_type].to_owned(),
                                    &token,
                                )],
                                expected_tokens,
                                source: None,
                            }
                            .into());
                        }
                    }
                    ParseType::N(n) => match self.predict_production(n, &stream) {
                        Ok(prod_num) => {
                            self.parser_stack.stack.pop();
                            self.push_production(&mut tree_builder, prod_num)?;
                        }
                        Err(source) => {
                            let nt_name = self.non_terminal_names[n];
                            let (message, unexpected_tokens, expected_tokens) = self
                                .lookahead_automata[n]
                                .build_error(self.terminal_names, &stream.borrow())?;
                            return Err(ParserError::PredictionErrorWithExpectations {
                                cause: self.diagnostic_message(
                                    format!(
                                        "{}\nat non-terminal \"{}\" \n\
                                            Current scanner is {}",
                                        message,
                                        nt_name,
                                        &stream.borrow().current_scanner(),
                                    )
                                    .as_str(),
                                ),
                                input: Box::new(FileSource::from_stream(&stream.borrow())),
                                error_location: unexpected_tokens
                                    .get(0)
                                    .map_or(Box::<Location>::default(), |t| {
                                        Box::new(t.token.clone())
                                    }),
                                unexpected_tokens,
                                expected_tokens,
                                source: Some(Box::new(source)),
                            }
                            .into());
                        }
                    },
                    ParseType::S(s) => {
                        stream.borrow_mut().switch_scanner(s)?;
                        self.parser_stack.stack.pop();
                    }
                    ParseType::Push(s) => {
                        trace!("%push({}) at production {:?}", s, self.current_production());
                        stream.borrow_mut().push_scanner(s)?;
                        self.parser_stack.stack.pop();
                    }
                    ParseType::Pop => {
                        trace!("%pop() at production {:?}", self.current_production());
                        let result = stream.borrow_mut().pop_scanner();
                        if let Err(source) = result {
                            return Err(ParserError::PopOnEmptyScannerStateStack {
                                context: self.diagnostic_message(
                                    format!(
                                        "Current scanner is {}",
                                        &stream.borrow().current_scanner(),
                                    )
                                    .as_str(),
                                ),
                                input: FileSource::from_stream(&stream.borrow()),
                                source,
                            }
                            .into());
                        }
                        self.parser_stack.stack.pop();
                    }
                    ParseType::E(p) => {
                        self.production_depth -= 1;
                        debug!("Popped production {} -> depth {}", p, self.production_depth);
                        self.parser_stack.stack.pop(); // Pop the End of production marker
                        self.process_item_stack(&mut tree_builder, p, user_actions)?;
                    }
                }
            }
        }

        if !stream.borrow().all_input_consumed() {
            Err((ParserError::UnprocessedInput {
                input: Box::new(FileSource::from_stream(&stream.borrow())),
                last_token: Box::new(stream.borrow().last_token()?.into()),
            })
            .into())
        } else {
            Ok(tree_builder
                .build()
                .map_err(|source| ParserError::TreeError { source })?)
        }
    }
}
