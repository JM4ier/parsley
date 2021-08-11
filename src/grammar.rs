#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NonTerminal(usize);

pub type Terminal = char;

pub enum Token {
    NT(NonTerminal),
    T(Terminal),
    None,
}

pub struct Rule {
    pub lhs: NonTerminal,
    pub rhs: Vec<Token>,
}

pub type Grammar = Vec<Rule>;

pub enum ChomskyRule {
    NonTerminal {
        lhs: NonTerminal,
        rhs: [NonTerminal; 2],
    },
    Terminal {
        lhs: NonTerminal,
        rhs: Terminal,
    },
}

pub type ChomskyGrammar = Vec<ChomskyRule>;
