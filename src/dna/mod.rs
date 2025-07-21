mod matchreplace;
mod pattern;
mod rope;
mod template;

use std::fmt;

pub use self::rope::{DnaRope, Iter as DnaRopeIter};
use crate::rna::Rna;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dna {
    I,
    C,
    F,
    P,
}

use self::Dna::*;

#[derive(Debug)]
pub enum ParseError {
    UnknownSymbol(char),
}

impl Dna {
    fn try_from(value: char) -> Result<Self, ParseError> {
        match value {
            'I' => Ok(I),
            'C' => Ok(C),
            'F' => Ok(F),
            'P' => Ok(P),
            _ => Err(ParseError::UnknownSymbol(value)),
        }
    }
}

impl fmt::Display for Dna {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn read_dna(dna_str: &str) -> Vec<Dna> {
    dna_str
        .chars()
        .map(Dna::try_from)
        .map(Result::unwrap)
        .collect()
}

pub struct DnaExecutor {
    dna: Option<DnaRope>,
    rna: Vec<Rna>,
    loops_count: usize,
}

impl DnaExecutor {
    pub fn execute(&mut self) -> &[Rna] {
        while let Some(dna) = self.dna.take() {
            let finished = self.execute_single(dna);
            if finished {
                break;
            }
        }
        self.rna()
    }

    pub fn execute_loops(&mut self, loops: usize) -> bool {
        let mut loops_done = 0;
        while let Some(dna) = self.dna.take() {
            let finished = self.execute_single(dna);
            loops_done += 1;
            if finished {
                return true;
            }
            if loops_done == loops {
                return false;
            }
        }
        true
    }

    pub fn rna(&self) -> &[Rna] {
        &self.rna[..]
    }

    fn add_rna(&mut self, rna: Rna) {
        self.rna.push(rna);
    }

    fn execute_single(&mut self, mut dna: DnaRope) -> bool {
        self.loops_count += 1;
        if self.loops_count % 60000 == 0 {
            debug!("running defragment");
            dna = dna.defragment();
        }
        debug!(
            "running loop: {} dna len: {} dna ropes count: {} rna len: {}",
            self.loops_count,
            dna.len(),
            dna.rope_count(),
            self.rna.len()
        );
        let (pattern, template, pos) = {
            let mut iter = dna.iter();
            let pos_pattern_start = iter.pos();
            if let Some(pattern) = pattern::execute(self, &mut iter) {
                let pos_pattern_end = iter.pos();
                trace!(
                    "pattern handled: {}",
                    debug(&dna.get_range(pos_pattern_start..pos_pattern_end))
                );
                let pos_template_start = pos_pattern_end;
                if let Some(template) = template::execute(self, &mut iter) {
                    let pos_template_end = iter.pos();
                    trace!(
                        "template handled: {}",
                        debug(&dna.get_range(pos_template_start..pos_template_end))
                    );
                    (pattern, template, iter.pos())
                } else {
                    return true;
                }
            } else {
                return true;
            }
        };
        let len = dna.len();
        trace!("dna len: {}", len);
        debug!(
            "pattern: {:?} template: {:?} pos: {}",
            debug(&pattern),
            debug(&template),
            pos
        );
        dna = dna.split_off(pos);
        let len = dna.len();
        trace!("len {}", len);
        self.dna = Some(matchreplace::execute(pattern, template, dna));
        let len = self.dna.as_ref().map(|x| x.len()).unwrap();
        trace!("{}", len);
        false
    }
}

fn debug<T: ToString>(t: &[T]) -> String {
    t.iter().map(T::to_string).collect::<Vec<_>>().join("")
}

impl From<&str> for DnaExecutor {
    fn from(value: &str) -> Self {
        let dna = Some(DnaRope::from(read_dna(value)));
        DnaExecutor {
            dna,
            rna: vec![],
            loops_count: 0,
        }
    }
}

fn nat(dna_iter: &mut DnaRopeIter) -> Option<usize> {
    let mut bit_index = 0;
    let mut n = 0;
    loop {
        match dna_iter.next() {
            Some(P) => {
                return Some(n);
            }
            Some(I) | Some(F) => (),
            Some(C) => {
                if bit_index < 64 {
                    n |= 1 << bit_index;
                } else {
                    n = usize::MAX;
                }
            }
            _ => return None,
        }
        bit_index += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_read_dna() {
        assert_eq!(read_dna("ICFPC"), vec![I, C, F, P, C]);
    }

    fn check_e2e(source: &str, result: &str) {
        println!("dna {:?} result expected: {:?}", source, result);
        let mut dna_executor = DnaExecutor::from(source);
        dna_executor.execute_loops(1);
        let dna = dna_executor.dna.unwrap_or_else(|| DnaRope::from(vec![]));
        let dna_remainig = dna.as_vec();
        assert_eq!(
            dna_remainig,
            &read_dna(result).iter().collect::<Vec<_>>()[..]
        );
    }

    #[test]
    fn case_e2e_01() {
        check_e2e("IIPIPICPIICICIIFICCIFPPIICCFPC", "PICFC");
    }

    #[test]
    fn case_e2e_02() {
        check_e2e("IIPIPICPIICICIIFICCIFCCCPPIICCFPC", "PIICCFCFFPC");
    }

    #[test]
    fn case_e2e_03() {
        check_e2e("IIPIPIICPIICIICCIICFCFC", "I");
    }
}
