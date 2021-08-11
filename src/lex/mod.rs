mod test;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    String(String),
    RuleOpen,
    RuleClose,
    GroupOpen,
    GroupClose,
    OptOpen,
    OptClose,
    RepOpen,
    RepClose,
    Assign,
    Newline,
}

pub fn lex(i: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = i.chars();
    let mut acc = String::new();

    while let Some(ch) = chars.next() {
        use Token::*;
        let t = match ch {
            '<' => RuleOpen,
            '>' => RuleClose,
            '(' => GroupOpen,
            ')' => GroupClose,
            '[' => OptOpen,
            ']' => OptClose,
            '{' => RepOpen,
            '}' => RepClose,
            ':' => Assign,
            '\n' => Newline,
            '\\' => {
                if let Some(ch) = chars.next() {
                    acc.push(ch);
                }
                continue;
            }
            ' ' | '\t' => continue,
            a => {
                acc.push(a);
                continue;
            }
        };
        if acc.len() > 0 {
            tokens.push(Token::String(acc));
            acc = Default::default();
        }
        tokens.push(t);
    }
    if acc.len() > 0 {
        tokens.push(Token::String(acc));
    }
    tokens
}
