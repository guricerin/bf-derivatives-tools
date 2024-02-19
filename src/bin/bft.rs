use bf_derivatives_tools::parser;
use clap::Parser;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

/// brainfuck derivative translator
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// path/to/brainfuck/code/file
    #[arg()]
    code_path: PathBuf,

    /// path/to/translate/from/grammar/file
    #[arg(short = 'f')]
    from_grammar_path: Option<PathBuf>,

    /// path/to/translate/to/grammar/file
    #[arg(short = 't')]
    to_grammar_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let code = fs::read_to_string(args.code_path)?;
    let mut parser = parser::Parser::new(&code);

    match (args.from_grammar_path, args.to_grammar_path) {
        (Some(from_path), Some(to_path)) => {
            let data = fs::read_to_string(from_path)?;
            let from_grammar = serde_json::from_str(&data)?;
            let data = fs::read_to_string(to_path)?;
            let to_grammar = serde_json::from_str(&data)?;
            parser.translate(&from_grammar, &to_grammar);
        }
        (Some(from_path), None) => {
            let data = fs::read_to_string(from_path)?;
            let from_grammar = serde_json::from_str(&data)?;
            parser.translate_to_bf(&from_grammar);
        }
        (None, Some(to_path)) => {
            let data = fs::read_to_string(to_path)?;
            let to_grammar = serde_json::from_str(&data)?;
            parser.translate_from_bf(&to_grammar);
        }
        _ => {
            eprintln!("both -f and -t options can't be omitted.");
            std::process::exit(1);
        }
    }

    println!("{}", parser.code());
    Ok(())
}
