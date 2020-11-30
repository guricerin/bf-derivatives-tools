use bf_derivatives_tool::parser::*;
use clap::Clap;
use serde_json;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(name = env!("CARGO_BIN_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[clap(name = "brainfuck-src-code-file")]
    code_path: PathBuf,
    #[clap(short = 'f', name = "translate-from-grammar-file")]
    from_grammar_path: Option<PathBuf>,
    #[clap(short = 't', name = "translate-to-grammar-file")]
    to_grammar_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let code = fs::read_to_string(opts.code_path)?;
    let mut parser = Parser::new(&code);

    match (opts.from_grammar_path, opts.to_grammar_path) {
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
            println!("both -f and -t options can't be omitted.");
            std::process::exit(1);
        }
    }

    println!("{}", parser.code());
    Ok(())
}
