#[cfg(test)]
mod test;

use crate::bnf::*;
use crate::lex::{Location, Token};

macro_rules! consume {
    ($line:expr, $pat:tt) => {
        consume!($line, $pat, {}, $pat)
    };
    ($line:expr, $pat:pat, $block:stmt, $expect:expr) => {{
        if !$line.is_empty() {
            if let $pat = &$line[0].1 {
                *$line = &$line[1..];
                $block
            } else {
                return Err(ParseError {
                    expected: vec![$expect],
                    got: Some($line[0].1.clone()),
                    location: Some($line[0].0.clone()),
                });
            }
        } else {
            return Err(ParseError {
                expected: vec![$expect],
                got: None,
                location: None,
            });
        }
    }};
}

use std::error::Error;
use std::fmt;
#[derive(Debug, Clone)]
pub struct ParseError {
    expected: Vec<Token>,
    got: Option<Token>,
    location: Option<Location>,
}
impl ParseError {
    pub fn message(&self) -> String {
        let exp = self
            .expected
            .iter()
            .map(Token::to_expected)
            .collect::<Vec<_>>();
        let got = match &self.got {
            Some(token) => token.to_expected(),
            None => "nothing".into(),
        };

        if exp.is_empty() {
            format!("unexpected symbol {}", got)
        } else if exp.len() == 1 {
            format!("expected {}, found {}", &exp[0], got)
        } else {
            format!(
                "expected {}or{}, found {}",
                exp[1..]
                    .iter()
                    .map(|e| format!("{}, ", e))
                    .collect::<String>(),
                &exp[0],
                got
            )
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expected one of {:?}, got {:?}", self.expected, self.got)
    }
}

impl Error for ParseError {}

pub type ParseResult<T> = Result<T, ParseError>;

pub fn parse(tokens: &[(Location, Token)]) -> Result<Vec<BnfRule>, Vec<ParseError>> {
    let lines = tokens.split(|t| t.1 == Token::Newline);
    let mut rules = Vec::new();
    let mut errors = Vec::new();
    let mut line_count = 0;

    for mut line in lines {
        if line.is_empty() {
            continue;
        }
        line_count += 1;
        let parse_result = (|| {
            let line = &mut line;

            use Token::*;

            let name;
            consume!(line, RuleOpen);
            consume!(line, String(ref s), name = s.clone(), String("".into()));
            consume!(line, RuleClose);
            consume!(line, Assign);
            let def = rparse(line, None)?;
            let rule = BnfRule { name, def };
            rules.push(rule);
            Ok(())
        })();
        let parse_result = parse_result.map_err(|mut e: ParseError| {
            if e.location.is_none() {
                e.location = tokens
                    .iter()
                    .filter(|(_, t)| t == &Token::Newline)
                    .nth(line_count - 1)
                    .map(|(l, _)| l.clone());
            }
            e
        });

        if let Err(err) = parse_result {
            errors.push(err);
        }
    }

    if errors.is_empty() {
        Ok(rules)
    } else {
        Err(errors)
    }
}

fn rparse(tokens: &mut &[(Location, Token)], mut closing: Option<Token>) -> ParseResult<BnfPart> {
    use BnfPart::*;
    use Token::*;

    let mut acc = Vec::new();
    let mut alts = Vec::new();

    let push_acc = |acc: &mut Vec<_>, alts: &mut Vec<_>| {
        let alt = if acc.is_empty() {
            BnfPart::Empty
        } else if acc.len() == 1 {
            acc.clone().into_iter().next().unwrap()
        } else {
            BnfPart::Concat(acc.clone())
        };
        alts.push(alt);
        *acc = Vec::new();
    };

    while !tokens.is_empty() {
        let first = &tokens[0];
        *tokens = &tokens[1..];
        match &first.1 {
            RuleOpen => {
                consume!(
                    tokens,
                    String(ref s),
                    acc.push(BnfPart::Rule(s.clone())),
                    String("".into())
                );
                consume!(tokens, RuleClose);
            }
            GroupOpen => acc.push(rparse(tokens, Some(GroupClose))?),
            OptOpen => acc.push(BnfPart::Opt(rparse(tokens, Some(OptClose))?)),
            RepOpen => acc.push(BnfPart::Repeat(Box::new(rparse(tokens, Some(RepClose))?))),
            String(s) => acc.push(Literal(s.clone())),
            Alternative => push_acc(&mut acc, &mut alts),
            token => {
                if Some(token.clone()) == closing {
                    closing = None;
                    break;
                } else {
                    let expected = closing.iter().cloned().collect();
                    return Err(ParseError {
                        expected,
                        got: Some(token.clone()),
                        location: Some(first.0.clone()),
                    });
                }
            }
        }
    }

    if let Some(closing) = closing {
        return Err(ParseError {
            expected: vec![closing],
            got: None,
            location: None,
        });
    }

    push_acc(&mut acc, &mut alts);

    let res = if alts.is_empty() {
        BnfPart::Empty
    } else if alts.len() == 1 {
        alts[0].clone()
    } else {
        BnfPart::Choice(alts)
    };

    Ok(res)
}

pub fn format_errors(file: &str, source: &str, errors: Vec<ParseError>) -> String {
    let chars = source.chars().collect::<Vec<_>>();

    use console::Style;
    let error_style = Style::new().red().bold();
    let line_style = Style::new().blue().bold();

    errors
        .iter()
        .map(|e| {
            let location = match &e.location {
                Some(l) => l,
                None => return String::from("TODO"),
            };
            let (mut l_from, mut l_to) = location.clone().into_inner();
            while l_from > 0 && chars[l_from - 1] != '\n' {
                l_from -= 1;
            }
            while l_to < chars.len() && chars[l_to] != '\n' {
                l_to += 1;
            }
            let (mut c_from, mut c_to) = location.clone().into_inner();
            c_from -= l_from;
            c_to -= l_from;

            let line_chars = chars[l_from..l_to]
                .iter()
                .chain(Some(&' '))
                .collect::<String>();

            let line_number = 1 + chars[..l_from].iter().filter(|c| **c == '\n').count();
            let line_fmt = |l| line_style.apply_to(format!("{: >3} |   ", l));

            let context = format!(
                "{}{}:{}:{}\n{}\n{}{}{}{}\n{}\n\n",
                line_style.apply_to("   --> "),
                file,
                line_number,
                c_from + 1,
                line_fmt("".into()),
                line_fmt(line_number.to_string()),
                &line_chars[..c_from],
                error_style.apply_to(&line_chars[c_from..=c_to]),
                &line_chars[c_to + 1..],
                line_fmt("".into()),
            );

            format!(
                "{}: {}\n{}",
                error_style.apply_to("error"),
                e.message(),
                context
            )
        })
        .collect::<String>()
}
