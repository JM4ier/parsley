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
            Self::String(s) => s,
            Self::RuleOpen => "<",
            Self::RuleClose => ">",
            Self::GroupOpen => "(",
            Self::GroupClose => ")",
            Self::OptOpen => "[",
            Self::OptClose => "]",
            Self::RepOpen => "{",
            Self::RepClose => "}",
            Self::Alternative => "|",
            Self::Assign => "<=",
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
    use Token::*;

    let mut tokens = Vec::new();
    let chars = i.chars().collect::<Vec<_>>();
    let mut acc = std::string::String::new();
    let mut acc_begin = 0;
    let mut i = 0;

    while let Some(ch) = chars.get(i) {
        let from = i;
        let t = match ch {
            '<' => {
                if let Some('=') = chars.get(i+1) {
                    i += 1;
                    Assign
                } else {
                    RuleOpen
                }
            },
            '>' => RuleClose,
            '(' => GroupOpen,
            ')' => GroupClose,
            '[' => OptOpen,
            ']' => OptClose,
            '{' => RepOpen,
            '}' => RepClose,
            '|' => Alternative,
            '\n' => Newline,
            '\\' => {
                i += 1;
                if let Some(ch) = chars.get(i) {
                    acc.push(*ch);
                    i += 1;
                }
                continue;
            }
            ' ' | '\t' => {
                i += 1;
                continue;
            }
            a => {
                acc.push(*a);
                i += 1;
                continue;
            }
        };
        if !acc.is_empty() {
            tokens.push((acc_begin..=i - 1, String(acc)));
            acc = Default::default();
        }
        acc_begin = i + 1;
        tokens.push((from..=i, t));
        i += 1;
    }

    if !acc.is_empty() {
        tokens.push((acc_begin..=i - 1, String(acc)));
    }
    tokens
}
