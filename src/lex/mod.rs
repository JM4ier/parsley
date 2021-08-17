#[cfg(test)]
mod test;

pub type Location = std::ops::RangeInclusive<usize>;

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

impl ToString for Token {
    fn to_string(&self) -> String {
        let s = match self {
            Self::String(s) => &s,
            Self::RuleOpen => "<",
            Self::RuleClose => ">",
            Self::GroupOpen => "(",
            Self::GroupClose => ")",
            Self::OptOpen => "[",
            Self::OptClose => "]",
            Self::RepOpen => "{",
            Self::RepClose => "}",
            Self::Alternative => "|",
            Self::Assign => ":",
            Self::Newline => "\\n",
        };
        s.to_string()
    }
}

impl Token {
    pub fn to_expected(&self) -> String {
        if let Self::String(_) = self {
            String::from("a literal")
        } else {
            format!("`{}`", self.to_string())
        }
    }
}

pub fn lex(i: &str) -> Vec<(Location, Token)> {
    let mut tokens = Vec::new();
    let mut chars = i.chars().enumerate();
    let mut acc = String::new();
    let mut acc_begin = 0;
    let mut acc_end = 0;

    while let Some((loc, ch)) = chars.next() {
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
                if let Some((end, ch)) = chars.next() {
                    acc.push(ch);
                    acc_end = end;
                }
                continue;
            }
            ' ' | '\t' => continue,
            a => {
                acc.push(a);
                acc_end = loc;
                continue;
            }
        };
        acc_begin = loc + 1;
        if acc.len() > 0 {
            tokens.push((acc_begin..=acc_end, Token::String(acc)));
            acc = Default::default();
        }
        tokens.push((loc..=loc, t));
    }
    if acc.len() > 0 {
        tokens.push((acc_begin..=acc_end, Token::String(acc)));
    }
    tokens
}
