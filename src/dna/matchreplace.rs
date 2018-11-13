use super::{
    pattern::Pattern,
    template::Template,
    DnaRope,
    DNA::{self, *},
};

use std::ops::Range;

pub fn execute(pattern: Vec<Pattern>, template: Vec<Template>, dna: DnaRope) -> DnaRope {
    if let Some((pos, ref env)) = execute_match(pattern, &dna) {
        execute_replace(&template, pos, env, dna)
    } else {
        dna
    }
}

fn execute_match(pattern: Vec<Pattern>, dna: &DnaRope) -> Option<(usize, Vec<Range<usize>>)> {
    let mut e: Vec<Range<usize>> = vec![];
    let mut c: Vec<usize> = vec![];
    let mut iter = dna.iter();
    for p in pattern {
        match p {
            Pattern::Base(c) => match iter.next() {
                Some(dnac) if dnac == &c => (),
                _ => return None,
            },
            Pattern::Skip(n) => {
                let absolute_pos = iter.pos() + n;
                if absolute_pos > dna.len() {
                    return None;
                }
                dna.iter_seek(&mut iter, absolute_pos);
            }
            Pattern::Search(c) => {
                if !c.is_empty() {
                    if dna.len() - iter.pos() < c.len() {
                        return None;
                    }
                    if let Some(n) = iter.dna_search(&c) {
                        dna.iter_seek(&mut iter, n + c.len());
                    } else {
                        return None;
                    }
                }
            }
            Pattern::GroupOpen => {
                c.push(iter.pos());
            }
            Pattern::GroupClose => {
                if let Some(start) = c.pop() {
                    let end = iter.pos();
                    e.push(if start < end { start..end } else { end..end });
                }
            }
        }
    }
    Some((iter.pos(), e))
}

fn execute_replace(
    template: &[Template],
    pos: usize,
    e: &[Range<usize>],
    mut dna: DnaRope,
) -> DnaRope {
    let mut postfix = dna.split_off(pos);

    let mut prefix = DnaRope::new();
    let mut env = vec![];
    for t in template {
        if let Template::NumberLevel(n, _) = t {
            if let Some(env_n) = e.get(*n) {
                if env_n.start != env_n.end {
                    env.push(env_n.clone());
                }
            }
        }
    }
    let splitted_env = dna.split_by_ranges(&env);
    let mut env = splitted_env.into_iter();
    let mut r = vec![];
    for t in template {
        match t {
            Template::Base(gene) => r.push(gene.clone()),
            Template::NumberLevel(n, l) => {
                if let Some(e_original) = e.get(*n) {
                    if e_original.start != e_original.end {
                        let env_n = env.next().unwrap();
                        if *l == 0 {
                            if !r.is_empty() {
                                prefix.append_dna(r.drain(..).collect());
                            }
                            prefix.append(env_n);
                        } else {
                            r.extend(protect(*l, &env_n.iter().cloned().collect::<Vec<_>>()[..]));
                        }
                    }
                }
            }
            Template::Length(n) => {
                r.extend(asnat(e.get(*n).map(|x| x.len()).unwrap_or(0)));
            }
        }
    }
    if !r.is_empty() {
        prefix.append_dna(r);
    }
    postfix.prepend(prefix);
    postfix
}

fn protect(lvl: usize, gene: &[DNA]) -> Vec<DNA> {
    if lvl == 0 {
        return gene.to_vec();
    }
    let mut d = gene.to_vec();
    let mut count = lvl;
    while count > 0 {
        d = quote(d);
        count -= 1;
    }
    d
}

fn quote(gene: Vec<DNA>) -> Vec<DNA> {
    let mut res = vec![];
    for c in gene {
        match c {
            I => res.push(C),
            C => res.push(F),
            F => res.push(P),
            P => {
                res.push(I);
                res.push(C);
            }
        }
    }
    res
}

fn asnat(n: usize) -> Vec<DNA> {
    let mut res = vec![];
    let mut n = n;
    while n != 0 {
        res.push(if n % 2 == 0 { I } else { C });
        n /= 2;
    }
    res.push(P);
    res
}

/*

const EMPTY_DNA: &[DNA] = &[];

fn replace_dna_from_environment(
    dna_executor: &mut DnaExecutor,
    template: &[Template],
    env: &[Range<usize>],
    i: usize,
) {
    /*
    let mut r = vec![];
    for t in template {
        match t {
            Template::Base(gene) => r.push(gene.clone()),
            Template::NumberLevel(n, l) => {
                let env_n = env
                    .get(*n)
                    .map(|x| &dna_executor.dna[x.start..x.end])
                    .unwrap_or(EMPTY_DNA);
                if *l == 0 {
                    r.extend_from_slice(env_n);
                } else {
                    r.extend(protect(*l, env_n));
                }
            }
            Template::Length(n) => {
                r.extend(asnat(env.get(*n).map(|x| x.len()).unwrap_or(0)));
            }
        }
    }
    dna_executor.dna.drain(0..i);
    r.extend_from_slice(&dna_executor.dna);
    dna_executor.dna = r;
    */
    unimplemented!();
}

*/
