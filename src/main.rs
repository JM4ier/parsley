pub mod bnf;
pub mod chomsky;
pub mod grammar;
pub mod lex;
pub mod parse;

use rustop::opts;
use std::io::prelude::*;

fn main() -> Result<(), String> {
    let (args, _) = opts! {
        synopsis "This parses EBNF";
        param file: String, desc: "Input file name.";
    }.parse_or_exit();

    let ebnf = (|| {
        let mut file = std::fs::File::open(args.file)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(buf)
    })().map_err(|_:std::io::Error| "File error")?;

    let tokens = lex::lex(&ebnf);
    let tokens = tokens.into_iter().map(|(_, t)| t).collect::<Vec<_>>();
    let rules = parse::parse(&tokens)?;
    let mut grammar = bnf::to_grammar(&rules, &rules[0].name);
    println!("{:?}", grammar);
    grammar.normalize();
    let cgrammar = chomsky::Grammar::from_normalized(&grammar)?;
    println!("{:?}", cgrammar);

    Ok(())
}
