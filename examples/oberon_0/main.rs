extern crate parol_runtime;

mod oberon_0_grammar;
mod oberon_0_grammar_trait;
mod oberon_0_parser;

use crate::oberon_0_grammar::Oberon0Grammar;
use crate::oberon_0_parser::parse;
use log::debug;
use miette::{miette, IntoDiagnostic, Result, WrapErr};
use parol::generate_tree_layout;
use std::env;
use std::fs;

// To rebuild the parser sources from scratch use the command build_parsers.ps1

// To run the example
// cargo run --example oberon_0 -- .\examples\oberon_0\Sample.mod

fn main() -> Result<()> {
    // $env:RUST_LOG="parol_runtime=debug,oberon_0=debug"
    env_logger::init();
    debug!("env logger started");

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file_name = args[1].clone();
        let input = fs::read_to_string(file_name.clone())
            .into_diagnostic()
            .wrap_err(format!("Can't read file {}", file_name))?;
        let mut oberon_0_grammar = Oberon0Grammar::new();
        let syntax_tree = parse(&input, &file_name, &mut oberon_0_grammar)
            .wrap_err(format!("Failed parsing file {}", file_name))?;
        println!("\n{} successfully parsed!", file_name);
        println!("{}", oberon_0_grammar);
        generate_tree_layout(&syntax_tree, &file_name).wrap_err("Error generating tree layout")
    } else {
        Err(miette!("Please provide a file name as single parameter!"))
    }
}
