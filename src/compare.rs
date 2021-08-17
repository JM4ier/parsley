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

        let mut prod1 = Producer::new(gram1);
        let mut prod2 = Producer::new(gram2);

        for _ in 0..limit {
            prod1.next();
        }
        let words1 = prod1.all_buffered_words();

        if let Some(last) = words1.last() {
            while let Some(w) = prod2.next() {
                if w.len() >= last.len() {
                    break;
                }
            }
        } else {
            for _ in 0..limit {
                prod2.next();
            }
        }
        let words2 = prod2.all_buffered_words();

        let mut p1 = 0;
        let mut p2 = 0;

        while p1 < words1.len() && p2 < words2.len() {
            match compare(&words1[p1], &words2[p2]) {
                Ordering::Equal => {
                    both.push(words1[p1].clone());
                    p1 += 1;
                    p2 += 1;
                }
                Ordering::Less => {
                    first.push(words1[p1].clone());
                    p1 += 1;
                }
                Ordering::Greater => {
                    second.push(words2[p2].clone());
                    p2 += 1;
                }
            }
        }

        while p1 < words1.len() {
            first.push(words1[p1].clone());
            p1 += 1;
        }

        while p2 < words2.len() {
            second.push(words2[p2].clone());
            p2 += 1;
        }

        Self {
            first,
            second,
            both,
        }
    }
}
