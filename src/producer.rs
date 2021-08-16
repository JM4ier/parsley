use crate::chomsky::*;

pub struct Producer {
    grammar: Grammar,
    curr_len: usize,
}

impl Producer {
    pub fn new(grammar: Grammar) -> Self {
        Self { grammar, curr_len: 0 }
    }
}

impl Iterator for Producer {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.grammar.null {
            self.grammar.null = false;
            return Some(String::new());
        }
        todo!()
    }
}
