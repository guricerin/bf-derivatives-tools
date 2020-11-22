use super::grammar::*;
use super::token::*;
use regex::Regex;

// lazy_static! {
//     static ref
// }
//static ORG_GRAMMAR: Grammar = Grammar::new(">", "<", "+", "-", ".", ",", "[", "]");
// const BF_TOKENS: &[char] = &['>', '<', '+', '-', '.', ',', '[', ']'];

pub struct Parser {
    code: String,
    //grammar: Grammar,
}

impl Parser {
    pub fn new(code: &str) -> Self {
        // let code = Parser::replace_all(code, &grammar.rshift, BF_TOKENS[0]);
        Self {
            code: code.to_string(),
            //grammar: grammar.clone(),
        }
    }

    pub fn replace(&mut self, grammar: &Grammar) {
        for rshift in grammar.rshift.iter() {
            self.code = Parser::replace_all(&self.code, &rshift, ">");
        }
        for lshift in grammar.lshift.iter() {
            self.code = Parser::replace_all(&self.code, &lshift, "<");
        }
        for inc in grammar.inc.iter() {
            self.code = Parser::replace_all(&self.code, &inc, "+");
        }
        for dec in grammar.dec.iter() {
            self.code = Parser::replace_all(&self.code, &dec, "-");
        }
        for write in grammar.write.iter() {
            self.code = Parser::replace_all(&self.code, &write, ".");
        }
        for read in grammar.read.iter() {
            self.code = Parser::replace_all(&self.code, &read, ",");
        }
        for loop_begin in grammar.loop_begin.iter() {
            self.code = Parser::replace_all(&self.code, &loop_begin, "[");
        }
        for loop_end in grammar.loop_end.iter() {
            self.code = Parser::replace_all(&self.code, &loop_end, "]");
        }
    }

    fn replace_all(s: &str, before: &str, after: &str) -> String {
        let re = Regex::new(before).unwrap();
        let res = re.replace_all(s, after).to_string();
        res
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
