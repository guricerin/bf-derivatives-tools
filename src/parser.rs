use super::grammar::*;
use super::token::*;
use regex;
use regex::Regex;

pub struct Parser {
    code: String,
}

impl Parser {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
        }
    }

    pub fn translate(&mut self, grammar: &Grammar) {
        let pattern = Parser::make_pattern(grammar);
        let translated = Regex::new(&pattern)
            .unwrap()
            .find_iter(&self.code)
            .map(|mat| {
                let s = mat.as_str();
                if s == grammar.rshift {
                    ">"
                } else if s == grammar.lshift {
                    "<"
                } else if s == grammar.inc {
                    "+"
                } else if s == grammar.dec {
                    "-"
                } else if s == grammar.write {
                    "."
                } else if s == grammar.read {
                    ","
                } else if s == grammar.loop_begin {
                    "["
                } else if s == grammar.loop_end {
                    "]"
                } else {
                    ""
                }
            })
            .collect::<String>();
        self.code = translated;
    }

    fn make_pattern(grammar: &Grammar) -> String {
        let rshift = regex::escape(&grammar.rshift);
        let lshift = regex::escape(&grammar.lshift);
        let inc = regex::escape(&grammar.inc);
        let dec = regex::escape(&grammar.dec);
        let write = regex::escape(&grammar.write);
        let read = regex::escape(&grammar.read);
        let loop_begin = regex::escape(&grammar.loop_begin);
        let loop_end = regex::escape(&grammar.loop_end);
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            rshift, lshift, inc, dec, write, read, loop_begin, loop_end
        )
    }

    pub fn parse(&self) -> Vec<Token> {
        let tokens: Vec<_> = self
            .code
            .chars()
            .map(|c| match c {
                '>' => Token::RShift,
                '<' => Token::LShift,
                '+' => Token::Inc,
                '-' => Token::Dec,
                '.' => Token::Write,
                ',' => Token::Read,
                '[' => Token::LoopBegin,
                ']' => Token::LoopEnd,
                _ => Token::Ignore,
            })
            .collect();
        tokens
    }
}
