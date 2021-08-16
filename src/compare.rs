use crate::chomsky::*;
use crate::producer::*;

#[derive(Default, Clone, Debug)]
pub struct Comparison {
    pub first: Vec<Terminal>,
    pub second: Vec<Terminal>,
    pub both: Vec<Terminal>,
}

use std::cmp::Ordering;
fn compare(a: &Terminal, b: &Terminal) -> Ordering {
    if a.len() < b.len() {
        Ordering::Less
    } else if a.len() > b.len() {
        Ordering::Greater
    } else {
        a.cmp(b)
    }
}

impl Comparison {
    pub fn from_grammars(gram1: Grammar, gram2: Grammar, limit: usize) -> Self {
        let mut first = Vec::new();
        let mut second = Vec::new();
        let mut both = Vec::new();

        let prod1 = Producer::new(gram1).take(limit).collect::<Vec<_>>();
        let prod2 = Producer::new(gram2).take(limit).collect::<Vec<_>>();

        let mut p1 = 0;
        let mut p2 = 0;

        while p1 < prod1.len() && p2 < prod2.len() {
            match compare(&prod1[p1], &prod2[p2]) {
                Ordering::Equal => {
                    both.push(prod1[p1].clone());
                    p1 += 1;
                    p2 += 1;
                }
                Ordering::Less => {
                    first.push(prod1[p1].clone());
                    p1 += 1;
                }
                Ordering::Greater => {
                    second.push(prod2[p2].clone());
                    p2 += 1;
                }
            }
        }

        Self {
            first,
            second,
            both,
        }
    }
}
