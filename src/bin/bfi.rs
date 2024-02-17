use bf_derivatives_tools::interpreter;
use bf_derivatives_tools::parser;
use clap::Parser;
use serde_json;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::result::Result;

/// brainfuck derivative interpreter
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// path/to/brainfuck/code/file
    #[arg()]
    code_path: PathBuf,

    /// path/to/brainfuck/grammer/file
    #[arg(short = 'g')]
    grammar_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let code = fs::read_to_string(args.code_path)?;
    let mut parser = parser::Parser::new(&code);

    if let Some(grammar_path) = args.grammar_path {
        let data = fs::read_to_string(grammar_path)?;
        let grammar = serde_json::from_str(&data)?;
        parser.translate_to_bf(&grammar);
    }
    let tokens = parser.parse();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut interpreter = interpreter::Interpreter::new(30000, stdin, stdout);
    interpreter.run(&tokens)?;

    Ok(())
}
