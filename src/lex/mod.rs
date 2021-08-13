mod test;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Location {
    pub line: usize,
    pub from: usize,
    pub to: usize,
}

impl Location {
    pub fn new(line: usize, chr: usize) -> Self {
        Self {
            line,
            from: chr,
            to: chr,
        }
    }
}

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
    Alternative,
    Assign,
    Newline,
}

pub fn lex(i: &str) -> Vec<(Location, Token)> {
    let mut tokens = Vec::new();
    let mut chars = i.chars();
    let mut acc = String::new();

    let mut line = 1;
    let mut chr = 0;

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
            '|' => Alternative,
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
            tokens.push((Location::new(line, chr), Token::String(acc)));
            acc = Default::default();
        }
        tokens.push((Location::new(line, chr), t));
    }
    if acc.len() > 0 {
        tokens.push((Location::new(line, chr), Token::String(acc)));
    }
    tokens
}
