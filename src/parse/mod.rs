#[cfg(test)]
mod test;

use crate::bnf::*;
use crate::lex::{Location, Token};

macro_rules! consume {
    ($line:expr, $pat:tt) => {
        consume!($line, $pat, {}, $pat)
    };
    ($line:expr, $pat:pat, $block:stmt, $expect:expr) => {{
        if $line.len() > 0 {
            if let $pat = &$line[0].1 {
                *$line = &$line[1..];
                $block
            } else {
                Err(ParseError {
                    expected: vec![$expect],
                    got: Some($line[0].1.clone()),
                    location: Some($line[0].0.clone()),
                })?
            }
        } else {
            Err(ParseError {
                expected: vec![$expect],
                got: None,
                location: None,
            })?
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

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "expected one of {:?}, got {:?}", self.expected, self.got)
    }
}

impl Error for ParseError {}

pub type ParseResult<T> = Result<T, ParseError>;

pub fn parse(tokens: &[(Location, Token)]) -> ParseResult<Vec<BnfRule>> {
    let lines = tokens.split(|t| t.1 == Token::Newline);
    let mut rules = Vec::new();
    let mut line_count = 0;

    let parse_result = (|| {
        for mut line in lines {
            if line.len() == 0 {
                continue;
            }

            let line = &mut line;

            use Token::*;

            let mut name = Default::default();
            consume!(line, RuleOpen);
            consume!(line, String(ref s), name = s.clone(), String("".into()));
            consume!(line, RuleClose);
            consume!(line, Assign);
            let def = rparse(line, None)?;
            let rule = BnfRule { name, def };
            rules.push(rule);
            line_count += 1;
        }
        Ok(())
    })();

    parse_result.map_err(|mut e: ParseError| {
        if e.location.is_none() {
            e.location = tokens
                .iter()
                .filter(|(_, t)| t == &Token::Newline)
                .nth(line_count)
                .map(|(l, _)| l.clone());
        }
        e
    })?;

    Ok(rules)
}

fn rparse(tokens: &mut &[(Location, Token)], mut closing: Option<Token>) -> ParseResult<BnfPart> {
    use BnfPart::*;
    use Token::*;

    let mut acc = Vec::new();
    let mut alts = Vec::new();

    let push_acc = |acc: &mut Vec<_>, alts: &mut Vec<_>| {
        let alt = if acc.len() == 0 {
            BnfPart::Empty
        } else if acc.len() == 1 {
            acc.clone().into_iter().next().unwrap()
        } else {
            BnfPart::Concat(acc.clone())
        };
        alts.push(alt);
        *acc = Vec::new();
    };

    while tokens.len() > 0 {
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
                    Err(ParseError {
                        expected,
                        got: Some(token.clone()),
                        location: Some(first.0.clone()),
                    })?
                }
            }
        }
    }

    if let Some(closing) = closing {
        Err(ParseError {
            expected: vec![closing],
            got: None,
            location: None,
        })?
    }

    push_acc(&mut acc, &mut alts);

    let res = if alts.len() == 0 {
        BnfPart::Empty
    } else if alts.len() == 1 {
        alts[0].clone()
    } else {
        BnfPart::Choice(alts)
    };

    Ok(res)
}
