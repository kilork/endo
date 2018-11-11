use super::{
    nat, DnaExecutor, DnaRopeIter,
    DNA::{self, *},
};
use rna::RNA;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Base(DNA),
    Skip(usize),
    Search(Vec<DNA>),
    GroupOpen,
    GroupClose,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pattern::Base(dna) => dna.to_string(),
                Pattern::Skip(n) => format!("![{}]", n),
                Pattern::Search(dna) => format!(
                    "?[{}]",
                    dna.iter().map(|x| x.to_string()).collect::<String>()
                ),
                Pattern::GroupOpen => "(".into(),
                Pattern::GroupClose => ")".into(),
            }
        )
    }
}

pub fn execute(executor: &mut DnaExecutor, iter: &mut DnaRopeIter) -> Option<Vec<Pattern>> {
    let mut pattern = vec![];
    let mut lvl = 0;
    loop {
        match iter.next() {
            Some(C) => {
                pattern.push(Pattern::Base(I));
            }
            Some(F) => {
                pattern.push(Pattern::Base(C));
            }
            Some(P) => {
                pattern.push(Pattern::Base(F));
            }
            Some(I) => match iter.next() {
                Some(C) => {
                    pattern.push(Pattern::Base(P));
                }
                Some(P) => {
                    if let Some(n) = nat(iter) {
                        pattern.push(Pattern::Skip(n));
                    } else {
                        return None;
                    }
                }
                Some(F) => {
                    iter.next();
                    let c = consts(iter);
                    pattern.push(Pattern::Search(c));
                }
                Some(I) => match iter.next() {
                    Some(P) => {
                        lvl += 1;
                        pattern.push(Pattern::GroupOpen);
                    }
                    Some(C) | Some(F) => {
                        if lvl == 0 {
                            return Some(pattern);
                        } else {
                            lvl -= 1;
                            pattern.push(Pattern::GroupClose);
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

fn consts(iter: &mut DnaRopeIter) -> Vec<DNA> {
    let mut c = vec![];
    loop {
        match iter.next() {
            Some(C) => {
                c.push(I);
            }
            Some(F) => {
                c.push(C);
            }
            Some(P) => {
                c.push(F);
            }
            Some(I) => match iter.next() {
                Some(C) => {
                    c.push(P);
                }
                _ => {
                    iter.step_back();
                    iter.step_back();
                    return c;
                }
            },
            None => return c,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pattern_for_test<T: Sized>(dna: &str, f: fn(&mut DnaExecutor, &mut DnaRopeIter) -> T) -> T {
        let mut dna_executor = DnaExecutor::from(dna);
        let dna = dna_executor.dna.take().unwrap();
        let mut iter = dna.iter();
        f(&mut dna_executor, &mut iter)
    }

    fn pattern_run(dna: &str) -> Option<Vec<Pattern>> {
        pattern_for_test(dna, execute)
    }

    fn pattern_nat(dna: &str) -> Option<usize> {
        pattern_for_test(dna, |_, iter| nat(iter))
    }

    fn pattern_consts(dna: &str) -> Vec<DNA> {
        pattern_for_test(dna, |_, iter| consts(iter))
    }

    #[test]
    fn case_pattern_nat() {
        assert_eq!(pattern_nat("CCICP"), Some(11));
        assert_eq!(pattern_nat("CFFFFFFFIIIIIIIIIIIP"), Some(1));
        assert_eq!(
            pattern_nat("IIIIICIICCIIIIIIIIIIIIIIP"),
            Some(32 + 256 + 512)
        );
    }

    #[test]
    fn case_pattern_consts() {
        assert_eq!(pattern_consts("ICFPC"), vec![P, C, F, I]);
        assert_eq!(pattern_consts("PCIIF"), vec![F, I]);
    }

    #[test]
    fn case_pattern_run() {
        assert_eq!(pattern_run("CIIC"), Some(vec![Pattern::Base(I)]));
        assert_eq!(
            pattern_run("IIPIPICPIICICIIF"),
            Some(vec![
                Pattern::GroupOpen,
                Pattern::Skip(2),
                Pattern::GroupClose,
                Pattern::Base(P)
            ])
        );
    }

    #[test]
    fn case_pattern_selfcheck_start_prefix() {
        // (?[IFPP])F
        assert_eq!(
            pattern_run("IIPIFFCPICICIICPIICIPPPICIIC"),
            Some(vec![
                Pattern::GroupOpen,
                Pattern::Search(vec![I, F, P, P]),
                Pattern::GroupClose,
                Pattern::Base(F)
            ])
        )
    }
}
