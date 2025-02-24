// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, Trans};
use parol_runtime::{ParolError, ParseTree};
use parol_runtime::{TokenStream, Tokenizer};
use std::cell::RefCell;
use std::path::Path;

use crate::parser::parol_grammar::ParolGrammar;
use crate::parser::parol_grammar_trait::ParolGrammarAuto;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 38] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r###"%start"###,
    /*  6 */ r###"%title"###,
    /*  7 */ r###"%comment"###,
    /*  8 */ r###"%user_type"###,
    /*  9 */ r###"="###,
    /* 10 */ r###"%line_comment"###,
    /* 11 */ r###"%block_comment"###,
    /* 12 */ r###"%auto_newline_off"###,
    /* 13 */ r###"%auto_ws_off"###,
    /* 14 */ r###"%%"###,
    /* 15 */ r###"::"###,
    /* 16 */ r###":"###,
    /* 17 */ r###";"###,
    /* 18 */ r###"\|"###,
    /* 19 */ r###"<"###,
    /* 20 */ r###">"###,
    /* 21 */ r###""(\\.|[^\\])*?""###,
    /* 22 */ r###"'(\\'|[^'])*?'"###,
    /* 23 */ r###"\u{2F}(\\.|[^\\]|)*?\u{2F}"###,
    /* 24 */ r###"\("###,
    /* 25 */ r###"\)"###,
    /* 26 */ r###"\["###,
    /* 27 */ r###"\]"###,
    /* 28 */ r###"\{"###,
    /* 29 */ r###"\}"###,
    /* 30 */ r###"[a-zA-Z_][a-zA-Z0-9_]*"###,
    /* 31 */ r###"%scanner"###,
    /* 32 */ r###","###,
    /* 33 */ r###"%sc"###,
    /* 34 */ r###"%push"###,
    /* 35 */ r###"%pop"###,
    /* 36 */ r###"\^"###,
    /* 37 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 38] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "PercentStart",
    /*  6 */ "PercentTitle",
    /*  7 */ "PercentComment",
    /*  8 */ "PercentUserUnderscoreType",
    /*  9 */ "Equ",
    /* 10 */ "PercentLineUnderscoreComment",
    /* 11 */ "PercentBlockUnderscoreComment",
    /* 12 */ "PercentAutoUnderscoreNewlineUnderscoreOff",
    /* 13 */ "PercentAutoUnderscoreWsUnderscoreOff",
    /* 14 */ "PercentPercent",
    /* 15 */ "DoubleColon",
    /* 16 */ "Colon",
    /* 17 */ "Semicolon",
    /* 18 */ "Or",
    /* 19 */ "LT",
    /* 20 */ "GT",
    /* 21 */ "String",
    /* 22 */ "RawString",
    /* 23 */ "Regex",
    /* 24 */ "LParen",
    /* 25 */ "RParen",
    /* 26 */ "LBracket",
    /* 27 */ "RBracket",
    /* 28 */ "LBrace",
    /* 29 */ "RBrace",
    /* 30 */ "Identifier",
    /* 31 */ "PercentScanner",
    /* 32 */ "Comma",
    /* 33 */ "PercentSc",
    /* 34 */ "PercentPush",
    /* 35 */ "PercentPop",
    /* 36 */ "CutOperator",
    /* 37 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[usize; 32]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ r###"(//.*(\r\n|\r|\n|$))"###,
        /*  4 */ r###"((?ms)/\*.*?\*/)"###,
    ],
    &[
        5,  /* PercentStart */
        6,  /* PercentTitle */
        7,  /* PercentComment */
        8,  /* PercentUserUnderscoreType */
        9,  /* Equ */
        10, /* PercentLineUnderscoreComment */
        11, /* PercentBlockUnderscoreComment */
        12, /* PercentAutoUnderscoreNewlineUnderscoreOff */
        13, /* PercentAutoUnderscoreWsUnderscoreOff */
        14, /* PercentPercent */
        15, /* DoubleColon */
        16, /* Colon */
        17, /* Semicolon */
        18, /* Or */
        19, /* LT */
        20, /* GT */
        21, /* String */
        22, /* RawString */
        23, /* Regex */
        24, /* LParen */
        25, /* RParen */
        26, /* LBracket */
        27, /* RBracket */
        28, /* LBrace */
        29, /* RBrace */
        30, /* Identifier */
        31, /* PercentScanner */
        32, /* Comma */
        33, /* PercentSc */
        34, /* PercentPush */
        35, /* PercentPop */
        36, /* CutOperator */
    ],
);

const MAX_K: usize = 1;

pub const NON_TERMINALS: &[&str; 42] = &[
    /*  0 */ "ASTControl",
    /*  1 */ "Alternation",
    /*  2 */ "AlternationList",
    /*  3 */ "Alternations",
    /*  4 */ "AlternationsList",
    /*  5 */ "CutOperator",
    /*  6 */ "Declaration",
    /*  7 */ "DoubleColon",
    /*  8 */ "Factor",
    /*  9 */ "GrammarDefinition",
    /* 10 */ "GrammarDefinitionList",
    /* 11 */ "Group",
    /* 12 */ "Identifier",
    /* 13 */ "NonTerminal",
    /* 14 */ "NonTerminalOpt",
    /* 15 */ "Optional",
    /* 16 */ "Parol",
    /* 17 */ "Production",
    /* 18 */ "Prolog",
    /* 19 */ "PrologList",
    /* 20 */ "PrologList0",
    /* 21 */ "RawString",
    /* 22 */ "Regex",
    /* 23 */ "Repeat",
    /* 24 */ "ScannerDirectives",
    /* 25 */ "ScannerState",
    /* 26 */ "ScannerStateList",
    /* 27 */ "ScannerSwitch",
    /* 28 */ "ScannerSwitchOpt",
    /* 29 */ "SimpleToken",
    /* 30 */ "SimpleTokenOpt",
    /* 31 */ "StartDeclaration",
    /* 32 */ "StateList",
    /* 33 */ "StateListList",
    /* 34 */ "String",
    /* 35 */ "Symbol",
    /* 36 */ "TokenLiteral",
    /* 37 */ "TokenWithStates",
    /* 38 */ "TokenWithStatesOpt",
    /* 39 */ "UserTypeDeclaration",
    /* 40 */ "UserTypeName",
    /* 41 */ "UserTypeNameList",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 42] = &[
    /* 0 - "ASTControl" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 16, 2, 65), Trans(0, 36, 1, 64)],
        k: 1,
    },
    /* 1 - "Alternation" */
    LookaheadDFA {
        prod0: 23,
        transitions: &[],
        k: 0,
    },
    /* 2 - "AlternationList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 17, 2, 25),
            Trans(0, 18, 2, 25),
            Trans(0, 19, 1, 24),
            Trans(0, 21, 1, 24),
            Trans(0, 22, 1, 24),
            Trans(0, 23, 1, 24),
            Trans(0, 24, 1, 24),
            Trans(0, 25, 2, 25),
            Trans(0, 26, 1, 24),
            Trans(0, 27, 2, 25),
            Trans(0, 28, 1, 24),
            Trans(0, 29, 2, 25),
            Trans(0, 30, 1, 24),
            Trans(0, 33, 1, 24),
            Trans(0, 34, 1, 24),
            Trans(0, 35, 1, 24),
        ],
        k: 1,
    },
    /* 3 - "Alternations" */
    LookaheadDFA {
        prod0: 20,
        transitions: &[],
        k: 0,
    },
    /* 4 - "AlternationsList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 17, 2, 22),
            Trans(0, 18, 1, 21),
            Trans(0, 25, 2, 22),
            Trans(0, 27, 2, 22),
            Trans(0, 29, 2, 22),
        ],
        k: 1,
    },
    /* 5 - "CutOperator" */
    LookaheadDFA {
        prod0: 66,
        transitions: &[],
        k: 0,
    },
    /* 6 - "Declaration" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 6, 1, 7),
            Trans(0, 7, 2, 8),
            Trans(0, 8, 3, 9),
            Trans(0, 10, 4, 10),
            Trans(0, 11, 4, 10),
            Trans(0, 12, 4, 10),
            Trans(0, 13, 4, 10),
        ],
        k: 1,
    },
    /* 7 - "DoubleColon" */
    LookaheadDFA {
        prod0: 18,
        transitions: &[],
        k: 0,
    },
    /* 8 - "Factor" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 19, 4, 29),
            Trans(0, 21, 4, 29),
            Trans(0, 22, 4, 29),
            Trans(0, 23, 4, 29),
            Trans(0, 24, 1, 26),
            Trans(0, 26, 3, 28),
            Trans(0, 28, 2, 27),
            Trans(0, 30, 4, 29),
            Trans(0, 33, 4, 29),
            Trans(0, 34, 4, 29),
            Trans(0, 35, 4, 29),
        ],
        k: 1,
    },
    /* 9 - "GrammarDefinition" */
    LookaheadDFA {
        prod0: 15,
        transitions: &[],
        k: 0,
    },
    /* 10 - "GrammarDefinitionList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 0, 2, 17), Trans(0, 30, 1, 16)],
        k: 1,
    },
    /* 11 - "Group" */
    LookaheadDFA {
        prod0: 46,
        transitions: &[],
        k: 0,
    },
    /* 12 - "Identifier" */
    LookaheadDFA {
        prod0: 52,
        transitions: &[],
        k: 0,
    },
    /* 13 - "NonTerminal" */
    LookaheadDFA {
        prod0: 49,
        transitions: &[],
        k: 0,
    },
    /* 14 - "NonTerminalOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 16, 1, 50),
            Trans(0, 17, 2, 51),
            Trans(0, 18, 2, 51),
            Trans(0, 19, 2, 51),
            Trans(0, 21, 2, 51),
            Trans(0, 22, 2, 51),
            Trans(0, 23, 2, 51),
            Trans(0, 24, 2, 51),
            Trans(0, 25, 2, 51),
            Trans(0, 26, 2, 51),
            Trans(0, 27, 2, 51),
            Trans(0, 28, 2, 51),
            Trans(0, 29, 2, 51),
            Trans(0, 30, 2, 51),
            Trans(0, 33, 2, 51),
            Trans(0, 34, 2, 51),
            Trans(0, 35, 2, 51),
            Trans(0, 36, 1, 50),
        ],
        k: 1,
    },
    /* 15 - "Optional" */
    LookaheadDFA {
        prod0: 47,
        transitions: &[],
        k: 0,
    },
    /* 16 - "Parol" */
    LookaheadDFA {
        prod0: 0,
        transitions: &[],
        k: 0,
    },
    /* 17 - "Production" */
    LookaheadDFA {
        prod0: 19,
        transitions: &[],
        k: 0,
    },
    /* 18 - "Prolog" */
    LookaheadDFA {
        prod0: 1,
        transitions: &[],
        k: 0,
    },
    /* 19 - "PrologList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 6, 1, 4),
            Trans(0, 7, 1, 4),
            Trans(0, 8, 1, 4),
            Trans(0, 10, 1, 4),
            Trans(0, 11, 1, 4),
            Trans(0, 12, 1, 4),
            Trans(0, 13, 1, 4),
            Trans(0, 14, 2, 5),
            Trans(0, 31, 2, 5),
        ],
        k: 1,
    },
    /* 20 - "PrologList0" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 14, 2, 3), Trans(0, 31, 1, 2)],
        k: 1,
    },
    /* 21 - "RawString" */
    LookaheadDFA {
        prod0: 44,
        transitions: &[],
        k: 0,
    },
    /* 22 - "Regex" */
    LookaheadDFA {
        prod0: 45,
        transitions: &[],
        k: 0,
    },
    /* 23 - "Repeat" */
    LookaheadDFA {
        prod0: 48,
        transitions: &[],
        k: 0,
    },
    /* 24 - "ScannerDirectives" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 10, 1, 11),
            Trans(0, 11, 2, 12),
            Trans(0, 12, 3, 13),
            Trans(0, 13, 4, 14),
        ],
        k: 1,
    },
    /* 25 - "ScannerState" */
    LookaheadDFA {
        prod0: 53,
        transitions: &[],
        k: 0,
    },
    /* 26 - "ScannerStateList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 10, 1, 54),
            Trans(0, 11, 1, 54),
            Trans(0, 12, 1, 54),
            Trans(0, 13, 1, 54),
            Trans(0, 29, 2, 55),
        ],
        k: 1,
    },
    /* 27 - "ScannerSwitch" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 33, 1, 59),
            Trans(0, 34, 2, 60),
            Trans(0, 35, 3, 61),
        ],
        k: 1,
    },
    /* 28 - "ScannerSwitchOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 25, 2, 63), Trans(0, 30, 1, 62)],
        k: 1,
    },
    /* 29 - "SimpleToken" */
    LookaheadDFA {
        prod0: 37,
        transitions: &[],
        k: 0,
    },
    /* 30 - "SimpleTokenOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 16, 1, 38),
            Trans(0, 17, 2, 39),
            Trans(0, 18, 2, 39),
            Trans(0, 19, 2, 39),
            Trans(0, 21, 2, 39),
            Trans(0, 22, 2, 39),
            Trans(0, 23, 2, 39),
            Trans(0, 24, 2, 39),
            Trans(0, 25, 2, 39),
            Trans(0, 26, 2, 39),
            Trans(0, 27, 2, 39),
            Trans(0, 28, 2, 39),
            Trans(0, 29, 2, 39),
            Trans(0, 30, 2, 39),
            Trans(0, 33, 2, 39),
            Trans(0, 34, 2, 39),
            Trans(0, 35, 2, 39),
            Trans(0, 36, 1, 38),
        ],
        k: 1,
    },
    /* 31 - "StartDeclaration" */
    LookaheadDFA {
        prod0: 6,
        transitions: &[],
        k: 0,
    },
    /* 32 - "StateList" */
    LookaheadDFA {
        prod0: 56,
        transitions: &[],
        k: 0,
    },
    /* 33 - "StateListList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 20, 2, 58), Trans(0, 32, 1, 57)],
        k: 1,
    },
    /* 34 - "String" */
    LookaheadDFA {
        prod0: 43,
        transitions: &[],
        k: 0,
    },
    /* 35 - "Symbol" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 19, 3, 32),
            Trans(0, 21, 2, 31),
            Trans(0, 22, 2, 31),
            Trans(0, 23, 2, 31),
            Trans(0, 30, 1, 30),
            Trans(0, 33, 4, 33),
            Trans(0, 34, 4, 33),
            Trans(0, 35, 4, 33),
        ],
        k: 1,
    },
    /* 36 - "TokenLiteral" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 21, 1, 34),
            Trans(0, 22, 2, 35),
            Trans(0, 23, 3, 36),
        ],
        k: 1,
    },
    /* 37 - "TokenWithStates" */
    LookaheadDFA {
        prod0: 40,
        transitions: &[],
        k: 0,
    },
    /* 38 - "TokenWithStatesOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 16, 1, 41),
            Trans(0, 17, 2, 42),
            Trans(0, 18, 2, 42),
            Trans(0, 19, 2, 42),
            Trans(0, 21, 2, 42),
            Trans(0, 22, 2, 42),
            Trans(0, 23, 2, 42),
            Trans(0, 24, 2, 42),
            Trans(0, 25, 2, 42),
            Trans(0, 26, 2, 42),
            Trans(0, 27, 2, 42),
            Trans(0, 28, 2, 42),
            Trans(0, 29, 2, 42),
            Trans(0, 30, 2, 42),
            Trans(0, 33, 2, 42),
            Trans(0, 34, 2, 42),
            Trans(0, 35, 2, 42),
            Trans(0, 36, 1, 41),
        ],
        k: 1,
    },
    /* 39 - "UserTypeDeclaration" */
    LookaheadDFA {
        prod0: 67,
        transitions: &[],
        k: 0,
    },
    /* 40 - "UserTypeName" */
    LookaheadDFA {
        prod0: 68,
        transitions: &[],
        k: 0,
    },
    /* 41 - "UserTypeNameList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 6, 2, 70),
            Trans(0, 7, 2, 70),
            Trans(0, 8, 2, 70),
            Trans(0, 10, 2, 70),
            Trans(0, 11, 2, 70),
            Trans(0, 12, 2, 70),
            Trans(0, 13, 2, 70),
            Trans(0, 14, 2, 70),
            Trans(0, 15, 1, 69),
            Trans(0, 17, 2, 70),
            Trans(0, 18, 2, 70),
            Trans(0, 19, 2, 70),
            Trans(0, 21, 2, 70),
            Trans(0, 22, 2, 70),
            Trans(0, 23, 2, 70),
            Trans(0, 24, 2, 70),
            Trans(0, 25, 2, 70),
            Trans(0, 26, 2, 70),
            Trans(0, 27, 2, 70),
            Trans(0, 28, 2, 70),
            Trans(0, 29, 2, 70),
            Trans(0, 30, 2, 70),
            Trans(0, 31, 2, 70),
            Trans(0, 33, 2, 70),
            Trans(0, 34, 2, 70),
            Trans(0, 35, 2, 70),
        ],
        k: 1,
    },
];

pub const PRODUCTIONS: &[Production; 71] = &[
    // 0 - Parol: Prolog GrammarDefinition;
    Production {
        lhs: 16,
        production: &[ParseType::N(9), ParseType::N(18)],
    },
    // 1 - Prolog: StartDeclaration PrologList /* Vec */ PrologList0 /* Vec */;
    Production {
        lhs: 18,
        production: &[ParseType::N(20), ParseType::N(19), ParseType::N(31)],
    },
    // 2 - PrologList0: ScannerState : crate::parser::parol_grammar::ScannerConfig  PrologList0;
    Production {
        lhs: 20,
        production: &[ParseType::N(20), ParseType::N(25)],
    },
    // 3 - PrologList0: ;
    Production {
        lhs: 20,
        production: &[],
    },
    // 4 - PrologList: Declaration PrologList;
    Production {
        lhs: 19,
        production: &[ParseType::N(19), ParseType::N(6)],
    },
    // 5 - PrologList: ;
    Production {
        lhs: 19,
        production: &[],
    },
    // 6 - StartDeclaration: '%start'^ /* Clipped */ Identifier;
    Production {
        lhs: 31,
        production: &[ParseType::N(12), ParseType::T(5)],
    },
    // 7 - Declaration: '%title'^ /* Clipped */ String;
    Production {
        lhs: 6,
        production: &[ParseType::N(34), ParseType::T(6)],
    },
    // 8 - Declaration: '%comment'^ /* Clipped */ String;
    Production {
        lhs: 6,
        production: &[ParseType::N(34), ParseType::T(7)],
    },
    // 9 - Declaration: '%user_type'^ /* Clipped */ Identifier '='^ /* Clipped */ UserTypeName : crate::parser::parol_grammar::UserDefinedTypeName ;
    Production {
        lhs: 6,
        production: &[
            ParseType::N(40),
            ParseType::T(9),
            ParseType::N(12),
            ParseType::T(8),
        ],
    },
    // 10 - Declaration: ScannerDirectives;
    Production {
        lhs: 6,
        production: &[ParseType::N(24)],
    },
    // 11 - ScannerDirectives: '%line_comment'^ /* Clipped */ TokenLiteral;
    Production {
        lhs: 24,
        production: &[ParseType::N(36), ParseType::T(10)],
    },
    // 12 - ScannerDirectives: '%block_comment'^ /* Clipped */ TokenLiteral TokenLiteral;
    Production {
        lhs: 24,
        production: &[ParseType::N(36), ParseType::N(36), ParseType::T(11)],
    },
    // 13 - ScannerDirectives: '%auto_newline_off'^ /* Clipped */;
    Production {
        lhs: 24,
        production: &[ParseType::T(12)],
    },
    // 14 - ScannerDirectives: '%auto_ws_off'^ /* Clipped */;
    Production {
        lhs: 24,
        production: &[ParseType::T(13)],
    },
    // 15 - GrammarDefinition: '%%'^ /* Clipped */ Production GrammarDefinitionList /* Vec */;
    Production {
        lhs: 9,
        production: &[ParseType::N(10), ParseType::N(17), ParseType::T(14)],
    },
    // 16 - GrammarDefinitionList: Production GrammarDefinitionList;
    Production {
        lhs: 10,
        production: &[ParseType::N(10), ParseType::N(17)],
    },
    // 17 - GrammarDefinitionList: ;
    Production {
        lhs: 10,
        production: &[],
    },
    // 18 - DoubleColon: '::';
    Production {
        lhs: 7,
        production: &[ParseType::T(15)],
    },
    // 19 - Production: Identifier ':'^ /* Clipped */ Alternations ';'^ /* Clipped */;
    Production {
        lhs: 17,
        production: &[
            ParseType::T(17),
            ParseType::N(3),
            ParseType::T(16),
            ParseType::N(12),
        ],
    },
    // 20 - Alternations: Alternation AlternationsList /* Vec */;
    Production {
        lhs: 3,
        production: &[ParseType::N(4), ParseType::N(1)],
    },
    // 21 - AlternationsList: '|'^ /* Clipped */ Alternation AlternationsList;
    Production {
        lhs: 4,
        production: &[ParseType::N(4), ParseType::N(1), ParseType::T(18)],
    },
    // 22 - AlternationsList: ;
    Production {
        lhs: 4,
        production: &[],
    },
    // 23 - Alternation: AlternationList /* Vec */;
    Production {
        lhs: 1,
        production: &[ParseType::N(2)],
    },
    // 24 - AlternationList: Factor AlternationList;
    Production {
        lhs: 2,
        production: &[ParseType::N(2), ParseType::N(8)],
    },
    // 25 - AlternationList: ;
    Production {
        lhs: 2,
        production: &[],
    },
    // 26 - Factor: Group;
    Production {
        lhs: 8,
        production: &[ParseType::N(11)],
    },
    // 27 - Factor: Repeat;
    Production {
        lhs: 8,
        production: &[ParseType::N(23)],
    },
    // 28 - Factor: Optional;
    Production {
        lhs: 8,
        production: &[ParseType::N(15)],
    },
    // 29 - Factor: Symbol;
    Production {
        lhs: 8,
        production: &[ParseType::N(35)],
    },
    // 30 - Symbol: NonTerminal;
    Production {
        lhs: 35,
        production: &[ParseType::N(13)],
    },
    // 31 - Symbol: SimpleToken;
    Production {
        lhs: 35,
        production: &[ParseType::N(29)],
    },
    // 32 - Symbol: TokenWithStates;
    Production {
        lhs: 35,
        production: &[ParseType::N(37)],
    },
    // 33 - Symbol: ScannerSwitch;
    Production {
        lhs: 35,
        production: &[ParseType::N(27)],
    },
    // 34 - TokenLiteral: String;
    Production {
        lhs: 36,
        production: &[ParseType::N(34)],
    },
    // 35 - TokenLiteral: RawString;
    Production {
        lhs: 36,
        production: &[ParseType::N(21)],
    },
    // 36 - TokenLiteral: Regex;
    Production {
        lhs: 36,
        production: &[ParseType::N(22)],
    },
    // 37 - SimpleToken: TokenLiteral SimpleTokenOpt /* Option */;
    Production {
        lhs: 29,
        production: &[ParseType::N(30), ParseType::N(36)],
    },
    // 38 - SimpleTokenOpt: ASTControl;
    Production {
        lhs: 30,
        production: &[ParseType::N(0)],
    },
    // 39 - SimpleTokenOpt: ;
    Production {
        lhs: 30,
        production: &[],
    },
    // 40 - TokenWithStates: '<'^ /* Clipped */ StateList '>'^ /* Clipped */ TokenLiteral TokenWithStatesOpt /* Option */;
    Production {
        lhs: 37,
        production: &[
            ParseType::N(38),
            ParseType::N(36),
            ParseType::T(20),
            ParseType::N(32),
            ParseType::T(19),
        ],
    },
    // 41 - TokenWithStatesOpt: ASTControl;
    Production {
        lhs: 38,
        production: &[ParseType::N(0)],
    },
    // 42 - TokenWithStatesOpt: ;
    Production {
        lhs: 38,
        production: &[],
    },
    // 43 - String: /"(\\.|[^\\])*?"/;
    Production {
        lhs: 34,
        production: &[ParseType::T(21)],
    },
    // 44 - RawString: /'(\\'|[^'])*?'/;
    Production {
        lhs: 21,
        production: &[ParseType::T(22)],
    },
    // 45 - Regex: /\u{2F}(\\.|[^\\]|)*?\u{2F}/;
    Production {
        lhs: 22,
        production: &[ParseType::T(23)],
    },
    // 46 - Group: '(' Alternations ')';
    Production {
        lhs: 11,
        production: &[ParseType::T(25), ParseType::N(3), ParseType::T(24)],
    },
    // 47 - Optional: '[' Alternations ']';
    Production {
        lhs: 15,
        production: &[ParseType::T(27), ParseType::N(3), ParseType::T(26)],
    },
    // 48 - Repeat: '{' Alternations '}';
    Production {
        lhs: 23,
        production: &[ParseType::T(29), ParseType::N(3), ParseType::T(28)],
    },
    // 49 - NonTerminal: Identifier NonTerminalOpt /* Option */;
    Production {
        lhs: 13,
        production: &[ParseType::N(14), ParseType::N(12)],
    },
    // 50 - NonTerminalOpt: ASTControl;
    Production {
        lhs: 14,
        production: &[ParseType::N(0)],
    },
    // 51 - NonTerminalOpt: ;
    Production {
        lhs: 14,
        production: &[],
    },
    // 52 - Identifier: /[a-zA-Z_][a-zA-Z0-9_]*/;
    Production {
        lhs: 12,
        production: &[ParseType::T(30)],
    },
    // 53 - ScannerState: '%scanner'^ /* Clipped */ Identifier '{'^ /* Clipped */ ScannerStateList /* Vec */ '}'^ /* Clipped */;
    Production {
        lhs: 25,
        production: &[
            ParseType::T(29),
            ParseType::N(26),
            ParseType::T(28),
            ParseType::N(12),
            ParseType::T(31),
        ],
    },
    // 54 - ScannerStateList: ScannerDirectives ScannerStateList;
    Production {
        lhs: 26,
        production: &[ParseType::N(26), ParseType::N(24)],
    },
    // 55 - ScannerStateList: ;
    Production {
        lhs: 26,
        production: &[],
    },
    // 56 - StateList: Identifier StateListList /* Vec */;
    Production {
        lhs: 32,
        production: &[ParseType::N(33), ParseType::N(12)],
    },
    // 57 - StateListList: ','^ /* Clipped */ Identifier StateListList;
    Production {
        lhs: 33,
        production: &[ParseType::N(33), ParseType::N(12), ParseType::T(32)],
    },
    // 58 - StateListList: ;
    Production {
        lhs: 33,
        production: &[],
    },
    // 59 - ScannerSwitch: '%sc'^ /* Clipped */ '('^ /* Clipped */ ScannerSwitchOpt /* Option */ ')'^ /* Clipped */;
    Production {
        lhs: 27,
        production: &[
            ParseType::T(25),
            ParseType::N(28),
            ParseType::T(24),
            ParseType::T(33),
        ],
    },
    // 60 - ScannerSwitch: '%push'^ /* Clipped */ '('^ /* Clipped */ Identifier ')'^ /* Clipped */;
    Production {
        lhs: 27,
        production: &[
            ParseType::T(25),
            ParseType::N(12),
            ParseType::T(24),
            ParseType::T(34),
        ],
    },
    // 61 - ScannerSwitch: '%pop'^ /* Clipped */ '('^ /* Clipped */ ')'^ /* Clipped */;
    Production {
        lhs: 27,
        production: &[ParseType::T(25), ParseType::T(24), ParseType::T(35)],
    },
    // 62 - ScannerSwitchOpt: Identifier;
    Production {
        lhs: 28,
        production: &[ParseType::N(12)],
    },
    // 63 - ScannerSwitchOpt: ;
    Production {
        lhs: 28,
        production: &[],
    },
    // 64 - ASTControl: CutOperator;
    Production {
        lhs: 0,
        production: &[ParseType::N(5)],
    },
    // 65 - ASTControl: UserTypeDeclaration;
    Production {
        lhs: 0,
        production: &[ParseType::N(39)],
    },
    // 66 - CutOperator: '^'^ /* Clipped */;
    Production {
        lhs: 5,
        production: &[ParseType::T(36)],
    },
    // 67 - UserTypeDeclaration: ':'^ /* Clipped */ UserTypeName : crate::parser::parol_grammar::UserDefinedTypeName ;
    Production {
        lhs: 39,
        production: &[ParseType::N(40), ParseType::T(16)],
    },
    // 68 - UserTypeName: Identifier UserTypeNameList /* Vec */;
    Production {
        lhs: 40,
        production: &[ParseType::N(41), ParseType::N(12)],
    },
    // 69 - UserTypeNameList: DoubleColon^ /* Clipped */ Identifier UserTypeNameList;
    Production {
        lhs: 41,
        production: &[ParseType::N(41), ParseType::N(12), ParseType::N(7)],
    },
    // 70 - UserTypeNameList: ;
    Production {
        lhs: 41,
        production: &[],
    },
];

static TOKENIZERS: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| {
    vec![(
        "INITIAL",
        Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
    )]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut ParolGrammar<'t>,
) -> Result<ParseTree<'t>, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        16,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    let token_stream =
        RefCell::new(TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap());
    // Initialize wrapper
    let mut user_actions = ParolGrammarAuto::new(user_actions);
    llk_parser.parse(token_stream, &mut user_actions)
}
