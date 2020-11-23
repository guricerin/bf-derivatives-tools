use bf_derivatives_tool::interpreter::*;
use bf_derivatives_tool::parser::*;
use clap::Clap;
use serde_json;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::result::Result;

#[derive(Clap, Debug)]
#[clap(name = env!("CARGO_BIN_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[clap(name = "brainfuck-src-code-file")]
    code_path: PathBuf,
    #[clap(short = 'g', name = "brainfuck-grammar-file")]
    grammar_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let code = fs::read_to_string(opts.code_path)?;
    let mut parser = Parser::new(&code);

    if let Some(grammar_path) = opts.grammar_path {
        let data = fs::read_to_string(grammar_path)?;
        let grammar = serde_json::from_str(&data)?;
        parser.translate_to_bf(&grammar);
    }
    let tokens = parser.parse();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut interpreter = Interpreter::new(30000, stdin, stdout);
    interpreter.run(&tokens)?;

    Ok(())
}
