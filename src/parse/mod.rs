#[cfg(test)]
mod test;

use crate::bnf::*;
use crate::lex::Token;

macro_rules! consume {
    ($line:expr, $pat:pat) => {
        consume!($line, $pat, {})
    };
    ($line:expr, $pat:pat, $block:stmt) => {{
        if $line.len() > 0 {
            if let $pat = &$line[0] {
                *$line = &$line[1..];
                $block
            } else {
                Err("expected a token, too lazy for proper error message.")?;
            }
        } else {
            Err("expected a token, too lazy for proper error message.")?;
        }
    }};
}

pub fn parse(tokens: &[Token]) -> Result<Vec<BnfRule>, String> {
    let lines = tokens.split(|t| *t == Token::Newline);
    let mut rules = Vec::new();
    for mut line in lines {
        if line.len() == 0 {
            continue;
        }

        let line = &mut line;

        use Token::*;

        let mut name = Default::default();
        consume!(line, RuleOpen);
        consume!(line, String(ref s), name = s.clone());
        consume!(line, RuleClose);
        consume!(line, Assign);
        let def = rparse(line, None)?;
        let rule = BnfRule { name, def };
        rules.push(rule);
    }
    Ok(rules)
}

fn rparse(tokens: &mut &[Token], closing: Option<Token>) -> Result<BnfPart, String> {
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
        match first {
            RuleOpen => {
                consume!(tokens, String(ref s), acc.push(BnfPart::Rule(s.clone())));
                consume!(tokens, RuleClose);
            }
            GroupOpen => acc.push(rparse(tokens, Some(GroupClose))?),
            OptOpen => acc.push(BnfPart::Opt(rparse(tokens, Some(OptClose))?)),
            RepOpen => acc.push(BnfPart::Repeat(Box::new(rparse(tokens, Some(RepClose))?))),
            String(s) => acc.push(Literal(s.clone())),
            Alternative => push_acc(&mut acc, &mut alts),
            token => {
                if Some(token.clone()) == closing {
                    break;
                } else {
                    Err(format!("unexpected token `{:?}`", token))?;
                }
            }
        }
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
