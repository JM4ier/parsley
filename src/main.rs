pub mod bnf;
pub mod chomsky;
pub mod compare;
pub mod grammar;
pub mod lex;
pub mod log;
pub mod parse;
pub mod producer;
pub mod styles;

use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Parses the file with the ebnf rule and reports any errors")]
    Parse { file: PathBuf },
    #[structopt(about = "Checks a word against the rules in the given file")]
    Check { file: PathBuf, word: String },
    #[structopt(
        about = "Checks words given in a file against the ebnf rules, separated by newline"
    )]
    CheckFile { rules: PathBuf, words: PathBuf },
    #[structopt(
        about = "Compares two sets of ebnf rules, and reports any words that are only accepted by either"
    )]
    Compare {
        file: PathBuf,
        other_file: PathBuf,
        #[structopt(default_value = "1000")]
        limit: usize,
    },
    #[structopt(about = "Gives a list of words produced by the ebnf rules, increasing in length")]
    ProduceWords {
        file: PathBuf,
        #[structopt(default_value = "20")]
        limit: usize,
    },
}

fn parse(file: &Path) -> chomsky::Grammar {
    let path = file.as_os_str().to_string_lossy();
    let ebnf = read_file(file);

    let tokens = lex::lex(&ebnf);

    let rules = match parse::parse(&tokens) {
        Ok(rules) => rules,
        Err(errs) => {
            print!("{}", parse::format_errors(&path, &ebnf, errs));
            println!(
                "{}: aborting due to previous errors",
                styles::ERROR.apply_to("error")
            );
            std::process::exit(1);
        }
    };
    debugln!("{:?}\n", rules);
    let mut grammar = bnf::to_grammar(&rules, &rules[0].name);
    grammar.simplify();
    grammar.normalize();
    debugln!("{}", grammar);
    match chomsky::Grammar::from_normalized(&grammar) {
        Ok(grammar) => grammar,
        Err(err) => {
            println!(
                "{}",
                styles::ERROR.apply_to("internal error: failed to normalize grammar")
            );
            debugln!("{:?}", err);
            std::process::exit(-1);
        }
    }
}

fn read_file(file: &Path) -> String {
    fn inner(file: &Path) -> std::io::Result<String> {
        let mut file = std::fs::File::open(file)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }
    match inner(file) {
        Ok(content) => content,
        Err(_) => {
            println!(
                "{}: failed to read file {}",
                styles::ERROR.apply_to("error"),
                file.as_os_str().to_string_lossy()
            );
            std::process::exit(2)
        }
    }
}

fn main() {
    let args = Options::from_args();

    log::enable(args.debug);

    use Command::*;
    match args.cmd {
        Parse { file } => {
            parse(&file);
            println!("{}", styles::GOOD.apply_to("No syntax errors were found."))
        }
        Check { file, word } => {
            let grammar = parse(&file);
            let verdict = if grammar.accepts(&word) {
                styles::GOOD.apply_to("accepted")
            } else {
                styles::ERROR.apply_to("rejected")
            };
            println!("`{}` is {} by this grammar.", word, verdict);
        }
        CheckFile { rules, words } => {
            let grammar = parse(&rules);
            let words = read_file(&words);
            let words = words.split('\n').collect::<Vec<_>>();

            let mut accepted = 0;
            for word in words.iter() {
                let accepts = grammar.accepts(word) as usize;
                let (yn, style) = [("n", &*styles::ERROR), ("y", &*styles::GOOD)][accepts];
                accepted += accepts;
                println!("{} '{}'", style.apply_to(format!("[{}]", yn)), word);
            }
            println!();
            println!(
                "{}",
                styles::INFO.apply_to(format!(
                    "{} out of {} words are recognized by this grammar.",
                    accepted,
                    words.len()
                ))
            );
        }
        Compare {
            file,
            other_file,
            limit,
        } => {
            let grammar = parse(&file);
            let other_grammar = parse(&other_file);
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
        ProduceWords { file, limit } => {
            let grammar = parse(&file);
            let words = producer::Producer::new(grammar)
                .map(|w| w.into_iter().collect::<String>())
                .take(limit)
                .collect::<Vec<_>>();
            println!("\nwords accepted by this grammar:");
            println!("{:?}", words);
        }
    }
}
