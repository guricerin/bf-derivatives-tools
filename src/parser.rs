use super::grammar::*;
use super::token::*;
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

    pub fn replace(&mut self, grammar: &Grammar) {
        for rshift in grammar.rshift.iter() {
            self.replace_all(&rshift, ">");
        }
        for lshift in grammar.lshift.iter() {
            self.replace_all(&lshift, "<");
        }
        for inc in grammar.inc.iter() {
            self.replace_all(&inc, "+");
        }
        for dec in grammar.dec.iter() {
            self.replace_all(&dec, "-");
        }
        for write in grammar.write.iter() {
            self.replace_all(&write, ".");
        }
        for read in grammar.read.iter() {
            self.replace_all(&read, ",");
        }
        for loop_begin in grammar.loop_begin.iter() {
            self.replace_all(&loop_begin, "[");
        }
        for loop_end in grammar.loop_end.iter() {
            self.replace_all(&loop_end, "]");
        }
    }

    fn replace_all(&mut self, before: &str, after: &str) {
        let re = Regex::new(before).unwrap();
        let res = re.replace_all(&self.code, after).to_string();
        self.code = res;
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
