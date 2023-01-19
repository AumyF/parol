use std::process;

use parol::{build::Builder, ParolErrorReporter};
use parol_runtime::Report;

fn main() {
    // CLI equivalent is:
    // parol -f ./snapshot_bin.par -e ./snapshot_bin-exp.par -p ./src/snapshot_bin_parser.rs -a ./src/snapshot_bin_grammar_trait.rs -t SnapshotBinGrammar -m snapshot_bin_grammar -g
    if let Err(err) = Builder::with_explicit_output_dir("src")
        .grammar_file("snapshot_bin.par")
        .expanded_grammar_output_file("../snapshot_bin-exp.par")
        .parser_output_file("snapshot_bin_parser.rs")
        .actions_output_file("snapshot_bin_grammar_trait.rs")
        .enable_auto_generation()
        .user_type_name("SnapshotBinGrammar")
        .user_trait_module_name("snapshot_bin_grammar")
        .generate_parser()
    {
        ParolErrorReporter::report_error(&err, "snapshot_bin.par").unwrap_or_default();
        process::exit(1);
    }
}
