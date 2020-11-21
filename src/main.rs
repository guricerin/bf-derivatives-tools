extern crate pest;
#[macro_use]
extern crate pest_derive;

use brainfuck_maker::interpreter::*;
use brainfuck_maker::token::*;
use pest::Parser;
use std::result::Result;

#[derive(Parser)]
#[grammar = "brainfuck.pest"]
struct BFParser;

fn main() -> Result<(), &'static str> {
    let program =
        "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.>++++++++++.";
    let tokens = parse(program);
    let mut interpreter = Interpreter::new(30000);
    interpreter.run(&tokens)?;
    Ok(())
}

fn parse(program: &str) -> Vec<Token> {
    let pair = BFParser::parse(Rule::tokens, program)
        .unwrap_or_else(|e| panic!("bf syntax error {}", e))
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
