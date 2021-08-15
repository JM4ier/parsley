use crate::grammar;
pub use grammar::{NonTerminal, Terminal};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Definition {
    Term(Terminal),
    Product([NonTerminal; 2]),
}

pub type Rule = Vec<Definition>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grammar {
    pub start: usize,
    pub null: bool,
    pub rules: Vec<Rule>,
}

impl Grammar {
    pub fn from_normalized(grammar: &grammar::Grammar) -> Result<Self, String> {
        use grammar::Token;

        let mut null = false;
        let rules = grammar
            .rules
            .iter()
            .enumerate()
            .map(|(idx, rul)| {
                Ok(rul
                    .iter()
                    .filter_map(|def| {
                        if def.len() == 0 {
                            if idx == grammar.start {
                                null = true;
                                None
                            } else {
                                Some(Err("only the starting rule may produce the empty symbol."))
                            }
                        } else if def.len() == 1 {
                            match &def[0] {
                                Token::NT(_) => Some(Err("Unit productions aren't allowed.")),
                                Token::T(t) => Some(Ok(Definition::Term(t.clone()))),
                            }
                        } else if def.len() == 2 {
                            match (&def[0], &def[1]) {
                                (Token::NT(t0), Token::NT(t1)) => {
                                    Some(Ok(Definition::Product([*t0, *t1])))
                                }
                                _ => Some(Err(
                                    "2-token definitions must consist of two nonterminals.",
                                )),
                            }
                        } else {
                            Some(Err("rules can't contain more than two tokens."))
                        }
                    })
                    .collect::<Result<_, &str>>()?)
            })
            .collect::<Result<Vec<Rule>, &str>>()?;
        Ok(Self {
            null,
            rules,
            start: grammar.start,
        })
    }

    pub fn accepts(&self, word: &str) -> bool {
        let chars = word.chars().collect::<Vec<_>>();
        #[allow(non_snake_case)]
        let N = chars.len();

        if N == 0 && self.null {
            return true;
        }

        let mut p = vec![vec![vec![false; N + 1]; N]; self.rules.len()];

        for (r, rule) in self.rules.iter().enumerate() {
            for def in rule.iter() {
                if let Definition::Term(term) = def {
                    for start in 0..(N - term.len() + 1) {
                        p[r][start][start + term.len()] |=
                            chars[start..start + term.len()] == term[..];
                    }
                }
            }
        }

        for len in 2..=N {
            for start in 0..N - len + 1 {
                for pivot in 0..len {
                    for (r, rule) in self.rules.iter().enumerate() {
                        for def in rule.iter() {
                            if let Definition::Product([c1, c2]) = *def {
                                p[r][start][start + len] |= //a
                                    p[c1][start][start + pivot]//a
                                    && p[c2][start + pivot][start + len];
                            }
                        }
                    }
                }
            }
        }

        p[self.start][0][N]
    }
}
