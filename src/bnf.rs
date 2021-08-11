#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BnfRule {
    pub name: String,
    pub def: BnfPart,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BnfPart {
    Empty,
    Literal(String),
    Choice(Vec<BnfPart>),
    Concat(Vec<BnfPart>),
    Repeat(Box<BnfPart>),
    Rule(String),
}

impl BnfPart {
    #[allow(non_snake_case)]
    pub fn Opt(part: Self) -> Self {
        Self::Choice(vec![part, Self::Empty])
    }
}
