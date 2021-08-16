pub mod bnf;
pub mod chomsky;
pub mod compare;
pub mod grammar;
pub mod lex;
pub mod parse;
pub mod producer;

use std::fs::*;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short, long)]
    debug: bool,
    file: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Parse,
    Check {
        word: String,
    },
    CheckFile {
        file: PathBuf,
    },
    CompareTo {
        other_file: PathBuf,
        #[structopt(default_value = "1000")]
        limit: usize,
    },
    ProduceWords {
        #[structopt(default_value = "20")]
        limit: usize,
    },
}

fn parse(file: &PathBuf, debug: bool) -> Result<chomsky::Grammar, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(file)?;
    let mut ebnf = String::new();
    file.read_to_string(&mut ebnf)?;

    let tokens = lex::lex(&ebnf);
    let rules = parse::parse(&tokens)?;
    if debug {
        println!("{:?}", rules);
    }
    let mut grammar = bnf::to_grammar(&rules, &rules[0].name);
    grammar.simplify();
    grammar.normalize();
    if debug {
        println!("{}", grammar);
    }
    Ok(chomsky::Grammar::from_normalized(&grammar)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Options::from_args();

    let grammar = parse(&args.file, args.debug)?;

    use Command::*;
    match args.cmd {
        Parse => (),
        Check { word } => {
            let verdict = if grammar.accepts(&word) {
                "accepted"
            } else {
                "rejected"
            };
            println!("`{}` is {} by this grammar.", word, verdict);
        }
        CheckFile { file } => {
            let mut file = File::open(file)?;
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            let words = buf.split("\n");

            for word in words {
                let yn = ["n", "y"][grammar.accepts(word) as usize];
                println!("[{}] '{}'", yn, word)
            }
        }
        CompareTo { other_file, limit } => {
            let other_grammar = parse(&other_file, args.debug)?;
            let compare = compare::Comparison::from_grammars(grammar, other_grammar, limit);
            let mapped = |words: &[chomsky::Terminal]| {
                words
                    .iter()
                    .map(|cs| cs.iter().cloned().collect::<String>())
                    .collect::<Vec<_>>()
            };
            println!(
                "words only accepted by the first grammar:\n{:?}",
                mapped(&compare.first)
            );
            println!(
                "words only accepted by the second grammar:\n{:?}",
                mapped(&compare.second)
            );
        }
        ProduceWords { limit } => {
            let words = producer::Producer::new(grammar)
                .map(|w| w.into_iter().collect::<String>())
                .take(limit)
                .collect::<Vec<_>>();
            println!("\nwords accepted by this grammar:");
            println!("{:?}", words);
        }
    }

    Ok(())
}
