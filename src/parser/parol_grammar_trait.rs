// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use crate::parser::parol_grammar::ParolGrammar;
use id_tree::Tree;
use parol_runtime::parser::errors::*;
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType, UserActionsTrait};

///
/// The `ParolGrammarTrait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
///
pub trait ParolGrammarTrait {
    /// Semantic action for production 0:
    ///
    /// Parol: Prolog GrammarDefinition;
    ///
    fn parol_0(
        &mut self,
        _prolog_0: &ParseTreeStackEntry,
        _grammar_definition_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// Prolog: StartDeclaration PrologSuffix;
    ///
    fn prolog_1(
        &mut self,
        _start_declaration_0: &ParseTreeStackEntry,
        _prolog_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// PrologSuffix: PrologRest;
    ///
    fn prolog_suffix_2(
        &mut self,
        _prolog_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// PrologSuffix: ;
    ///
    fn prolog_suffix_3(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// PrologRest: GlobalDirective PrologRestSuffix;
    ///
    fn prolog_rest_4(
        &mut self,
        _global_directive_0: &ParseTreeStackEntry,
        _prolog_rest_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// PrologRestSuffix: PrologRest;
    ///
    fn prolog_rest_suffix_5(
        &mut self,
        _prolog_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// PrologRestSuffix: ;
    ///
    fn prolog_rest_suffix_6(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// GlobalDirective: Declaration;
    ///
    fn global_directive_7(
        &mut self,
        _declaration_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// GlobalDirective: ScannerState;
    ///
    fn global_directive_8(
        &mut self,
        _scanner_state_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 9:
    ///
    /// StartDeclaration: "%start" Identifier;
    ///
    fn start_declaration_9(
        &mut self,
        _end_of_input_0: &ParseTreeStackEntry,
        _identifier_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 10:
    ///
    /// Declaration: "%title" String;
    ///
    fn declaration_10(
        &mut self,
        _newline_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 11:
    ///
    /// Declaration: "%comment" String;
    ///
    fn declaration_11(
        &mut self,
        _whitespace_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 12:
    ///
    /// Declaration: ScannerDirectives;
    ///
    fn declaration_12(
        &mut self,
        _scanner_directives_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 13:
    ///
    /// ScannerDirectives: "%line_comment" String;
    ///
    fn scanner_directives_13(
        &mut self,
        _line_comment_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 14:
    ///
    /// ScannerDirectives: "%block_comment" String String;
    ///
    fn scanner_directives_14(
        &mut self,
        _block_comment_0: &ParseTreeStackEntry,
        _string_1: &ParseTreeStackEntry,
        _string_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 15:
    ///
    /// ScannerDirectives: "%auto_newline_off";
    ///
    fn scanner_directives_15(
        &mut self,
        _percent_auto_underscore_newline_underscore_off_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 16:
    ///
    /// ScannerDirectives: "%auto_ws_off";
    ///
    fn scanner_directives_16(
        &mut self,
        _percent_auto_underscore_ws_underscore_off_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 17:
    ///
    /// GrammarDefinition: "%%" Production GrammarDefinitionSuffix;
    ///
    fn grammar_definition_17(
        &mut self,
        _percent_percent_0: &ParseTreeStackEntry,
        _production_1: &ParseTreeStackEntry,
        _grammar_definition_suffix_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 18:
    ///
    /// GrammarDefinitionSuffix: GrammarDefinitionRest;
    ///
    fn grammar_definition_suffix_18(
        &mut self,
        _grammar_definition_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 19:
    ///
    /// GrammarDefinitionSuffix: ;
    ///
    fn grammar_definition_suffix_19(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 20:
    ///
    /// GrammarDefinitionRest: Production GrammarDefinitionRestSuffix;
    ///
    fn grammar_definition_rest_20(
        &mut self,
        _production_0: &ParseTreeStackEntry,
        _grammar_definition_rest_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 21:
    ///
    /// GrammarDefinitionRestSuffix: GrammarDefinitionRest;
    ///
    fn grammar_definition_rest_suffix_21(
        &mut self,
        _grammar_definition_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 22:
    ///
    /// GrammarDefinitionRestSuffix: ;
    ///
    fn grammar_definition_rest_suffix_22(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 23:
    ///
    /// Production: Identifier ":" Alternations ";";
    ///
    fn production_23(
        &mut self,
        _identifier_0: &ParseTreeStackEntry,
        _colon_1: &ParseTreeStackEntry,
        _alternations_2: &ParseTreeStackEntry,
        _semicolon_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 24:
    ///
    /// Alternations: Alternation AlternationsSuffix;
    ///
    fn alternations_24(
        &mut self,
        _alternation_0: &ParseTreeStackEntry,
        _alternations_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 25:
    ///
    /// AlternationsSuffix: AlternationsRest;
    ///
    fn alternations_suffix_25(
        &mut self,
        _alternations_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 26:
    ///
    /// AlternationsSuffix: ;
    ///
    fn alternations_suffix_26(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 27:
    ///
    /// AlternationsRest: "\|" Alternation AlternationsRestSuffix;
    ///
    fn alternations_rest_27(
        &mut self,
        _or_0: &ParseTreeStackEntry,
        _alternation_1: &ParseTreeStackEntry,
        _alternations_rest_suffix_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 28:
    ///
    /// AlternationsRestSuffix: AlternationsRest;
    ///
    fn alternations_rest_suffix_28(
        &mut self,
        _alternations_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 29:
    ///
    /// AlternationsRestSuffix: ;
    ///
    fn alternations_rest_suffix_29(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 30:
    ///
    /// Alternation: AlternationRest;
    ///
    fn alternation_30(
        &mut self,
        _alternation_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 31:
    ///
    /// Alternation: ;
    ///
    fn alternation_31(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 32:
    ///
    /// AlternationRest: Factor AlternationRestSuffix;
    ///
    fn alternation_rest_32(
        &mut self,
        _factor_0: &ParseTreeStackEntry,
        _alternation_rest_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 33:
    ///
    /// AlternationRestSuffix: AlternationRest;
    ///
    fn alternation_rest_suffix_33(
        &mut self,
        _alternation_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 34:
    ///
    /// AlternationRestSuffix: ;
    ///
    fn alternation_rest_suffix_34(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 35:
    ///
    /// Factor: Group;
    ///
    fn factor_35(
        &mut self,
        _group_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 36:
    ///
    /// Factor: Repeat;
    ///
    fn factor_36(
        &mut self,
        _repeat_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 37:
    ///
    /// Factor: Optional;
    ///
    fn factor_37(
        &mut self,
        _optional_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 38:
    ///
    /// Factor: Symbol;
    ///
    fn factor_38(
        &mut self,
        _symbol_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 39:
    ///
    /// Symbol: Identifier;
    ///
    fn symbol_39(
        &mut self,
        _identifier_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 40:
    ///
    /// Symbol: SimpleToken;
    ///
    fn symbol_40(
        &mut self,
        _simple_token_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 41:
    ///
    /// Symbol: TokenWithStates;
    ///
    fn symbol_41(
        &mut self,
        _token_with_states_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 42:
    ///
    /// SimpleToken: String;
    ///
    fn simple_token_42(
        &mut self,
        _string_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 43:
    ///
    /// TokenWithStates: "<" StateList ">" String;
    ///
    fn token_with_states_43(
        &mut self,
        _l_t_0: &ParseTreeStackEntry,
        _state_list_1: &ParseTreeStackEntry,
        _g_t_2: &ParseTreeStackEntry,
        _string_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 44:
    ///
    /// Group: "\(" Alternations "\)";
    ///
    fn group_44(
        &mut self,
        _l_paren_0: &ParseTreeStackEntry,
        _alternations_1: &ParseTreeStackEntry,
        _r_paren_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 45:
    ///
    /// Optional: "\[" Alternations "\]";
    ///
    fn optional_45(
        &mut self,
        _l_bracket_0: &ParseTreeStackEntry,
        _alternations_1: &ParseTreeStackEntry,
        _r_bracket_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 46:
    ///
    /// Repeat: "\{" Alternations "\}";
    ///
    fn repeat_46(
        &mut self,
        _l_brace_0: &ParseTreeStackEntry,
        _alternations_1: &ParseTreeStackEntry,
        _r_brace_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 47:
    ///
    /// Identifier: "[a-zA-Z_]\w*";
    ///
    fn identifier_47(
        &mut self,
        _identifier_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 48:
    ///
    /// String: "\u{0022}([^\\]|\\.)*?\u{0022}";
    ///
    fn string_48(
        &mut self,
        _string_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 49:
    ///
    /// ScannerState: "%scanner" Identifier "\{" ScannerStateSuffix;
    ///
    fn scanner_state_49(
        &mut self,
        _percent_scanner_0: &ParseTreeStackEntry,
        _identifier_1: &ParseTreeStackEntry,
        _l_brace_2: &ParseTreeStackEntry,
        _scanner_state_suffix_3: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 50:
    ///
    /// ScannerStateSuffix: ScannerStateRest "\}";
    ///
    fn scanner_state_suffix_50(
        &mut self,
        _scanner_state_rest_0: &ParseTreeStackEntry,
        _r_brace_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 51:
    ///
    /// ScannerStateSuffix: "\}";
    ///
    fn scanner_state_suffix_51(
        &mut self,
        _r_brace_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 52:
    ///
    /// ScannerStateRest: ScannerDirectives ScannerStateRestSuffix;
    ///
    fn scanner_state_rest_52(
        &mut self,
        _scanner_directives_0: &ParseTreeStackEntry,
        _scanner_state_rest_suffix_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 53:
    ///
    /// ScannerStateRestSuffix: ScannerStateRest;
    ///
    fn scanner_state_rest_suffix_53(
        &mut self,
        _scanner_state_rest_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 54:
    ///
    /// ScannerStateRestSuffix: ;
    ///
    fn scanner_state_rest_suffix_54(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 55:
    ///
    /// StateList: Identifier StateListRest;
    ///
    fn state_list_55(
        &mut self,
        _identifier_0: &ParseTreeStackEntry,
        _state_list_rest_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 56:
    ///
    /// StateListRest: "," Identifier StateListRest;
    ///
    fn state_list_rest_56(
        &mut self,
        _comma_0: &ParseTreeStackEntry,
        _identifier_1: &ParseTreeStackEntry,
        _state_list_rest_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 57:
    ///
    /// StateListRest: ;
    ///
    fn state_list_rest_57(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }
}

impl UserActionsTrait for ParolGrammar {
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeStackEntry],
        parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        match prod_num {
            0 => self.parol_0(&children[0], &children[1], parse_tree),

            1 => self.prolog_1(&children[0], &children[1], parse_tree),

            2 => self.prolog_suffix_2(&children[0], parse_tree),

            3 => self.prolog_suffix_3(parse_tree),

            4 => self.prolog_rest_4(&children[0], &children[1], parse_tree),

            5 => self.prolog_rest_suffix_5(&children[0], parse_tree),

            6 => self.prolog_rest_suffix_6(parse_tree),

            7 => self.global_directive_7(&children[0], parse_tree),

            8 => self.global_directive_8(&children[0], parse_tree),

            9 => self.start_declaration_9(&children[0], &children[1], parse_tree),

            10 => self.declaration_10(&children[0], &children[1], parse_tree),

            11 => self.declaration_11(&children[0], &children[1], parse_tree),

            12 => self.declaration_12(&children[0], parse_tree),

            13 => self.scanner_directives_13(&children[0], &children[1], parse_tree),

            14 => self.scanner_directives_14(&children[0], &children[1], &children[2], parse_tree),

            15 => self.scanner_directives_15(&children[0], parse_tree),

            16 => self.scanner_directives_16(&children[0], parse_tree),

            17 => self.grammar_definition_17(&children[0], &children[1], &children[2], parse_tree),

            18 => self.grammar_definition_suffix_18(&children[0], parse_tree),

            19 => self.grammar_definition_suffix_19(parse_tree),

            20 => self.grammar_definition_rest_20(&children[0], &children[1], parse_tree),

            21 => self.grammar_definition_rest_suffix_21(&children[0], parse_tree),

            22 => self.grammar_definition_rest_suffix_22(parse_tree),

            23 => self.production_23(
                &children[0],
                &children[1],
                &children[2],
                &children[3],
                parse_tree,
            ),

            24 => self.alternations_24(&children[0], &children[1], parse_tree),

            25 => self.alternations_suffix_25(&children[0], parse_tree),

            26 => self.alternations_suffix_26(parse_tree),

            27 => self.alternations_rest_27(&children[0], &children[1], &children[2], parse_tree),

            28 => self.alternations_rest_suffix_28(&children[0], parse_tree),

            29 => self.alternations_rest_suffix_29(parse_tree),

            30 => self.alternation_30(&children[0], parse_tree),

            31 => self.alternation_31(parse_tree),

            32 => self.alternation_rest_32(&children[0], &children[1], parse_tree),

            33 => self.alternation_rest_suffix_33(&children[0], parse_tree),

            34 => self.alternation_rest_suffix_34(parse_tree),

            35 => self.factor_35(&children[0], parse_tree),

            36 => self.factor_36(&children[0], parse_tree),

            37 => self.factor_37(&children[0], parse_tree),

            38 => self.factor_38(&children[0], parse_tree),

            39 => self.symbol_39(&children[0], parse_tree),

            40 => self.symbol_40(&children[0], parse_tree),

            41 => self.symbol_41(&children[0], parse_tree),

            42 => self.simple_token_42(&children[0], parse_tree),

            43 => self.token_with_states_43(
                &children[0],
                &children[1],
                &children[2],
                &children[3],
                parse_tree,
            ),

            44 => self.group_44(&children[0], &children[1], &children[2], parse_tree),

            45 => self.optional_45(&children[0], &children[1], &children[2], parse_tree),

            46 => self.repeat_46(&children[0], &children[1], &children[2], parse_tree),

            47 => self.identifier_47(&children[0], parse_tree),

            48 => self.string_48(&children[0], parse_tree),

            49 => self.scanner_state_49(
                &children[0],
                &children[1],
                &children[2],
                &children[3],
                parse_tree,
            ),

            50 => self.scanner_state_suffix_50(&children[0], &children[1], parse_tree),

            51 => self.scanner_state_suffix_51(&children[0], parse_tree),

            52 => self.scanner_state_rest_52(&children[0], &children[1], parse_tree),

            53 => self.scanner_state_rest_suffix_53(&children[0], parse_tree),

            54 => self.scanner_state_rest_suffix_54(parse_tree),

            55 => self.state_list_55(&children[0], &children[1], parse_tree),

            56 => self.state_list_rest_56(&children[0], &children[1], &children[2], parse_tree),

            57 => self.state_list_rest_57(parse_tree),

            _ => panic!("Unhandled production number: {}", prod_num),
        }
    }
}
