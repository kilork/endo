use super::{
    nat, DnaExecutor, DnaRopeIter,
    DNA::{self, *},
};
use rna::RNA;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Template {
    Base(DNA),
    NumberLevel(usize, usize),
    Length(usize),
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Template::Base(dna) => dna.to_string(),
                Template::NumberLevel(n, l) => if l == &0 { format!("({})", n) } else { format!("({},{})", n, l) },
                Template::Length(n) => format!("|{}|", n),
            }
        )
    }
}

pub fn execute(executor: &mut DnaExecutor, iter: &mut DnaRopeIter) -> Option<Vec<Template>> {
    let mut template = vec![];
    loop {
        match iter.next() {
            Some(C) => {
                template.push(Template::Base(I));
            }
            Some(F) => {
                template.push(Template::Base(C));
            }
            Some(P) => {
                template.push(Template::Base(F));
            }
            Some(I) => match iter.next() {
                Some(C) => {
                    template.push(Template::Base(P));
                }
                Some(F) | Some(P) => {
                    if let Some(l) = nat(iter) {
                        if let Some(n) = nat(iter) {
                            template.push(Template::NumberLevel(n, l));
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                Some(I) => match iter.next() {
                    Some(C) | Some(F) => {
                        return Some(template);
                    }
                    Some(P) => {
                        if let Some(n) = nat(iter) {
                            template.push(Template::Length(n));
                        } else {
                            return None;
                        }
                    }
                    Some(I) => {
                        executor.add_rna(RNA::from_dna_iter(iter));
                    }
                    _ => return None,
                },
                _ => return None,
            },
            _ => return None,
        }
    }
}
