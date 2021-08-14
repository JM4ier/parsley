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

pub fn to_grammar(rules: &[BnfRule]) -> Grammar {
    let mut grammar = Grammar::new();
    // rule name to nonterminal
    let mut rtn = HashMap::new();
    let mut defs = Vec::new();

    for rule in rules.iter() {
        if !rtn.contains_key(&rule.name) {
            let nt = grammar.add_rule(vec![]);
            rtn.insert(&rule.name, nt);
        }
        defs.push(rule.def.clone());
    }

    fn visit_part<'a>(part: &'a BnfPart, rtn: &mut HashMap<&'a String, NonTerminal>, g: &mut Grammar) {
        use BnfPart::*;
        match part {
            Rule(s) => {
                if !rtn.contains_key(s) {
                    let nt = g.add_rule(vec![]);
                    rtn.insert(s, nt);
                }
            }
            Choice(parts) => parts.iter().for_each(|p| visit_part(p, rtn, g)),
            Concat(parts) => parts.iter().for_each(|p| visit_part(p, rtn, g)),
            Repeat(part) => visit_part(part, rtn, g),
            _ => (),
        }
    }
    visit_part(&BnfPart::Concat(defs), &mut rtn, &mut grammar);

    todo!() 
}
