pub mod bnf;
pub mod chomsky;
pub mod grammar;
pub mod lex;
pub mod parse;
pub mod producer;

use rustop::opts;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (args, _) = opts! {
        synopsis "This parses EBNF";
        param file: String, desc: "Input file name.";
        param check: Option<String>, desc: "check string against rule";
    }
    .parse_or_exit();

    let mut file = std::fs::File::open(args.file)?;
    let mut ebnf = String::new();
    file.read_to_string(&mut ebnf)?;

    let tokens = lex::lex(&ebnf);
    let rules = parse::parse(&tokens)?;
    println!("{:?}", rules);
    let mut grammar = bnf::to_grammar(&rules, &rules[0].name);
    grammar.simplify();
    grammar.normalize();
    println!("{}", grammar);
    let cgrammar = chomsky::Grammar::from_normalized(&grammar)?;

    if let Some(check) = args.check {
        let verdict = if cgrammar.accepts(&check) {
            "accepted"
        } else {
            "rejected"
        };
        println!("`{}` is {} by these rules.", check, verdict);
    }

    println!("\nwords accepted by this grammar:");
    let words = producer::Producer::new(cgrammar)
        .map(|w| w.into_iter().collect::<String>())
        .take(10)
        .collect::<Vec<_>>();
    println!("{:?}", words);

    Ok(())
}
