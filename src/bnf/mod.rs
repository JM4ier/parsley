#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BnfRule {
    pub name: String,
    pub def: BnfPart,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BnfPart {
    Empty,
    Literal(String),
    Choice(Vec<BnfPart>),
    Concat(Vec<BnfPart>),
    Repeat(Box<BnfPart>),
    Rule(String),
}

impl BnfPart {
    #[allow(non_snake_case)]
    /// Constructs an optional value by adding the empty word as alternative
    pub fn Opt(part: Self) -> Self {
        Self::Choice(vec![part, Self::Empty])
    }
}

use crate::grammar::*;
use std::collections::*;

impl BnfPart {
    pub fn simplify(&mut self) {}
}

pub fn to_grammar(rules: &[BnfRule], root: &str) -> Grammar {
    fn convert<'a>(
        part: &'a BnfPart,
        lookup: &mut HashMap<&'a String, NonTerminal>,
        g: &mut Grammar,
    ) -> NonTerminal {
        use BnfPart::*;
        use Token::*;
        match part {
            Rule(s) => {
                if !lookup.contains_key(s) {
                    let nt = g.add_rule(vec![]);
                    lookup.insert(s, nt);
                    nt
                } else {
                    lookup[s]
                }
            }
            Choice(parts) => {
                let parts = parts
                    .iter()
                    .map(|p| vec![NT(convert(p, lookup, g))])
                    .collect::<Vec<_>>();
                g.add_rule(parts)
            }
            Concat(parts) => {
                let parts = parts
                    .iter()
                    .map(|p| NT(convert(p, lookup, g)))
                    .collect::<Vec<_>>();
                g.add_rule(vec![parts])
            }
            Repeat(part) => {
                let part = convert(part, lookup, g);
                let rule = g.add_rule(vec![]);
                g.rules[rule].push(vec![]);
                g.rules[rule].push(vec![NT(rule), NT(rule)]);
                g.rules[rule].push(vec![NT(part)]);
                rule
            }
            Literal(lit) => g.add_rule(vec![vec![T(lit.clone())]]),
            Empty => g.add_rule(vec![vec![]]),
        }
    }

    let mut grammar = Grammar::new();
    // rule name to nonterminal
    let mut lookup = HashMap::new();

    for rule in rules.iter() {
        let nt = convert(&rule.def, &mut lookup, &mut grammar);
        if let Some(&def) = lookup.get(&rule.name) {
            grammar.rules[def].push(vec![Token::NT(nt)]);
        } else {
            lookup.insert(&rule.name, nt);
        }
    }

    let root = root.to_string();

    if let Some(&root) = lookup.get(&root) {
        grammar.start = root;
    } else {
        grammar.start = grammar.add_rule(vec![]);
    }

    grammar
}
