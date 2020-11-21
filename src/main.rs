extern crate pest;
#[macro_use]
extern crate pest_derive;

use brainfuck_maker::interpreter::*;
use brainfuck_maker::token::*;
use clap::Clap;
use pest::Parser;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::result::Result;

#[derive(Clap, Debug)]
#[clap(name = "brainfuck-maker", version = "0.1.0", author = "guricerin")]
struct Opts {
    #[clap(short, long, name = "brainfuck src code file")]
    input: Option<PathBuf>,
}

#[derive(Parser)]
#[grammar = "brainfuck.pest"]
struct BFParser;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    match opts.input {
        Some(src_file) => {
            rep(&src_file)?;
        }
        None => {
            repl()?;
        }
    }

    Ok(())
}

fn rep(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut interpreter = Interpreter::new(30000);
    let program = fs::read_to_string(path)?;
    let tokens = parse(&program);
    interpreter.run(&tokens)?;
    Ok(())
}

fn repl() -> Result<(), Box<dyn Error>> {
    let mut interpreter = Interpreter::new(30000);
    loop {
        print!("brainfuck>> ");
        let mut code = String::new();
        io::stdin().read_line(&mut code).ok();
        let code = code.trim();
        if code == "quit" {
            break;
        } else {
            let tokens = parse(&code);
            interpreter.run(&tokens)?;
        }
    }
    Ok(())
}

fn parse(program: &str) -> Vec<Token> {
    let pair = BFParser::parse(Rule::tokens, program)
        .unwrap_or_else(|e| panic!("bf syntax error: \n{}", e))
        .next()
        .unwrap();

    let v = pair
        .into_inner()
        .map(|token| match token.as_rule() {
            Rule::rshift => Token::RShift,
            Rule::lshift => Token::LShift,
            Rule::inc => Token::Inc,
            Rule::dec => Token::Dec,
            Rule::read => Token::Read,
            Rule::write => Token::Write,
            Rule::loopbegin => Token::LoopBegin,
            Rule::loopend => Token::LoopEnd,
            _ => unreachable!(),
        })
        .collect::<Vec<Token>>();
    v
}
