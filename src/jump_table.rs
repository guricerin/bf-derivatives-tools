use super::token::*;
use std::collections::HashMap;
use std::result::Result;

#[derive(Debug)]
pub struct JumpTable(HashMap<usize, usize>);

impl JumpTable {
    pub fn new(tokens: &Vec<Token>) -> Result<Self, &'static str> {
        let mut begins = vec![];
        let mut map = HashMap::<usize, usize>::new();
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::LoopBegin => begins.push(i),
                Token::LoopEnd => {
                    if let Some(j) = begins.pop() {
                        map.entry(i).or_insert(j);
                        map.entry(j).or_insert(i);
                    } else {
                        return Err("loop-end-bracket has not pair!");
                    }
                }
                _ => (),
            }
        }

        if begins.is_empty() {
            Ok(Self(map))
        } else {
            Err("blacket is not balanced!")
        }
    }

    pub fn get(&self, key: usize) -> Option<&usize> {
        self.0.get(&key)
    }
}
