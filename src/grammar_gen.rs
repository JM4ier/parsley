use crate::bnf::*;
use crate::grammar::*;
use std::collections::*;

fn bnf_to_grammar(rules: Vec<BnfRule>) -> Grammar {
    let mut rule_map = HashMap::new();
    for rule in rules.iter() {
        rule_map.insert(&rule.name, rule_map.len());
    }

    todo!()
}
