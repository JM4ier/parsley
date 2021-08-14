pub mod bnf;
pub mod chomsky;
pub mod grammar;
pub mod lex;
pub mod parse;

use rustop::opts;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (args, _) = opts! {
        synopsis "This parses EBNF";
        param file: String, desc: "Input file name.";
    }
    .parse_or_exit();

    let mut file = std::fs::File::open(args.file)?;
    let mut ebnf = String::new();
    file.read_to_string(&mut ebnf)?;

    let tokens = lex::lex(&ebnf);
    let rules = parse::parse(&tokens)?;
    println!("{:?}", rules);
    let mut grammar = bnf::to_grammar(&rules, &rules[0].name);
    println!("{}", grammar);
    grammar.simplify();
    println!("{}", grammar);
    grammar.normalize();
    println!("{}", grammar);
    let cgrammar = chomsky::Grammar::from_normalized(&grammar)?;
    println!("{:?}", cgrammar);

    Ok(())
}
