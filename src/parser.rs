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

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn translate_to_bf(&mut self, from: &Grammar) {
        let to = Grammar::bf();
        self.translate(from, &to);
    }

    pub fn translate_from_bf(&mut self, to: &Grammar) {
        let from = Grammar::bf();
        self.translate(&from, &to);
    }

    pub fn translate(&mut self, from: &Grammar, to: &Grammar) {
        let pattern = Parser::make_pattern(from);
        let translated = Regex::new(&pattern)
            .unwrap()
            .find_iter(&self.code)
            .map(|mat| {
                let s = mat.as_str();
                if s == from.rshift {
                    &to.rshift
                } else if s == from.lshift {
                    &to.lshift
                } else if s == from.inc {
                    &to.inc
                } else if s == from.dec {
                    &to.dec
                } else if s == from.write {
                    &to.write
                } else if s == from.read {
                    &to.read
                } else if s == from.loop_begin {
                    &to.loop_begin
                } else if s == from.loop_end {
                    &to.loop_end
                } else if s == " " || s == "\t" || s == "\n" || s == "\r\n" {
                    s
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
            "{}|{}|{}|{}|{}|{}|{}|{}|\x20|\t|\n|\r\n",
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
