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
        let code = Parser::replace_all(&self.code, &grammar.rshift, ">");
        let code = Parser::replace_all(&code, &grammar.lshift, "<");
        let code = Parser::replace_all(&code, &grammar.inc, "+");
        let code = Parser::replace_all(&code, &grammar.dec, "-");
        let code = Parser::replace_all(&code, &grammar.write, ".");
        let code = Parser::replace_all(&code, &grammar.read, ",");
        let code = Parser::replace_all(&code, &grammar.loop_begin, "[");
        let code = Parser::replace_all(&code, &grammar.loop_end, "]");
        self.code = code;
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
