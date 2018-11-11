use super::DNA;
use std::collections::VecDeque;

use std::cmp::Ordering;
use std::ops::Range;

#[derive(Clone)]
pub struct DnaRope {
    dna: VecDeque<Vec<DNA>>,
    index: VecDeque<usize>,
}

impl From<Vec<DNA>> for DnaRope {
    fn from(value: Vec<DNA>) -> Self {
        Self::from_raw(vec![value].into())
    }
}

impl DnaRope {
    pub fn new() -> Self {
        Self {
            dna: VecDeque::new(),
            index: VecDeque::new(),
        }
    }

    pub fn from_raw(dna: VecDeque<Vec<DNA>>) -> Self {
        let index = Self::create_index(&dna);
        Self { dna, index }
    }

    fn create_index(dna: &VecDeque<Vec<DNA>>) -> VecDeque<usize> {
        let mut index = VecDeque::with_capacity(dna.len());
        let mut count = 0;
        for subdna in dna {
            count += subdna.len();
            index.push_back(count);
        }
        index
    }

    pub fn prepend(&mut self, mut prefix: DnaRope) {
        while let Some(subdna) = prefix.dna.pop_back() {
            self.dna.push_front(subdna);
        }
        self.index = Self::create_index(&self.dna);
    }

    pub fn append_dna(&mut self, subdna: Vec<DNA>) {
        let count = self.len() + subdna.len();
        self.dna.push_back(subdna);
        self.index.push_back(count);
    }

    pub fn append(&mut self, mut suffix: DnaRope) {
        while let Some(subdna) = suffix.dna.pop_front() {
            self.append_dna(subdna);
        }
    }

    pub fn get_range(&self, range: Range<usize>) -> Vec<&DNA> {
        self.iter()
            .skip(range.start)
            .take(range.end - range.start)
            .collect::<Vec<_>>()
    }

    pub fn as_vec(&self) -> Vec<&DNA> {
        self.iter().collect()
    }

    pub fn len(&self) -> usize {
        *self.index.back().unwrap_or(&0)
    }

    pub fn rope_count(&self) -> usize {
        self.dna.len()
    }

    pub fn defragment(self) -> DnaRope {
        let dna: Vec<DNA> = self.iter().map(DNA::clone).collect();
        DnaRope::from(dna)
    }

    pub fn iter(&self) -> Iter {
        Iter {
            rope: &self,
            index: 0,
            vec: 0,
            absolute_index: 0,
        }
    }

    pub fn iter_seek(&self, iter: &mut Iter, n: usize) {
        if let Some((vec, index)) = self.index_pair(n) {
            iter.vec = vec;
            iter.index = index;
            iter.absolute_index = n;
        } else {
            iter.vec = 0;
            iter.index = 0;
            iter.absolute_index = self.len();
        }
    }

    pub fn split_off(&mut self, at: usize) -> Self {
        if let Some((vec, at)) = self.index_pair(at) {
            let mut postfix = self.dna.split_off(vec);
            self.index.split_off(vec);

            if at > 0 {
                let vec_postfix = postfix[0].split_off(at);
                let vec_suffix = std::mem::replace(&mut postfix[0], vec_postfix);
                let vec_suffix_len = vec_suffix.len();
                self.dna.push_back(vec_suffix);

                let last_index = self.len() + vec_suffix_len;
                self.index.push_back(last_index);
            }

            Self::from_raw(postfix)
        } else {
            Self::from(vec![])
        }
    }

    fn index_pair(&self, i: usize) -> Option<(usize, usize)> {
        let index = &self.index;
        match index.iter().collect::<Vec<&usize>>().binary_search(&&i) {
            Ok(vec) => if vec + 1 < self.dna.len() {
                Some((vec + 1, 0))
            } else {
                None
            },
            Err(vec) => Some((vec, i + self.dna[vec].len() - index[vec])),
        }
    }

    pub fn split_by_ranges(mut self, ranges: &[Range<usize>]) -> Vec<DnaRope> {
        let mut ranges: Vec<(usize, &Range<usize>)> = ranges.iter().enumerate().collect();
        ranges.sort_unstable_by(|a, b| match a.1.start.cmp(&b.1.start) {
            Ordering::Equal => b.1.end.cmp(&a.1.end),
            x => x,
        });

        let mut result: Vec<(usize, Option<Vec<DNA>>, &Range<usize>)> = vec![];
        for range in ranges {
            let have_intersection = result
                .iter()
                .any(|x| !((range.1.start >= x.2.end) || (range.1.end <= x.2.start)));
            result.push((
                range.0,
                if have_intersection {
                    Some(self.copy_from_range(range.1.clone()))
                } else {
                    None
                },
                range.1,
            ));
        }

        let mut dna_ropes = vec![];
        while let Some(mut range) = result.pop() {
            dna_ropes.push((
                range.0,
                if let Some(dna) = range.1.take() {
                    DnaRope::from(dna)
                } else {
                    if range.2.end < self.len() {
                        self.split_off(range.2.end);
                    }
                    self.split_off(range.2.start)
                },
            ));
        }
        dna_ropes.sort_unstable_by_key(|x| x.0);

        dna_ropes.into_iter().map(|x| x.1).collect()
    }

    fn copy_from_range(&self, range: Range<usize>) -> Vec<DNA> {
        self.iter_from_range(range).map(DNA::clone).collect()
    }

    fn iter_from_range(&self, range: Range<usize>) -> RangeIter {
        if let Some((vec, index)) = self.index_pair(range.start) {
            RangeIter {
                iter: Iter {
                    rope: &self,
                    vec,
                    index,
                    absolute_index: range.start,
                },
                remaining: range.end - range.start,
            }
        } else {
            panic!("Wrong range: {:?}", range);
        }
    }
}

struct RangeIter<'a> {
    iter: Iter<'a>,
    remaining: usize,
}

impl<'a> Iterator for RangeIter<'a> {
    type Item = &'a DNA;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        self.iter.next()
    }
}

#[derive(Clone)]
pub struct Iter<'a> {
    pub rope: &'a DnaRope,
    pub vec: usize,
    pub index: usize,
    pub absolute_index: usize,
}

impl<'a, 'b: 'a> Iter<'a> {
    pub fn dna_search(&self, key: &[DNA]) -> Option<usize> {
        if key.is_empty() {
            return None;
        }

        let key_len = key.len();
        if key_len == 1 {
            let key = key[0];
            let mut iter = self.clone();
            let pos = iter.pos();
            return iter.position(|&x| x == key).map(|x| pos + x);
        }

        let mut dfa = vec![[0; 4]; key_len];
        dfa[0][key[0].clone() as usize] = 1;
        let mut x = 0;
        for j in 1..key_len {
            dfa[j] = dfa[x];
            let k = key[j].clone() as usize;
            dfa[j][k] = j + 1;
            x = dfa[x][k];
        }

        let mut iter = self.clone();
        let mut j = 0;
        while let Some(dna) = iter.next() {
            j = dfa[j][dna.clone() as usize];
            if j == key_len {
                return Some(iter.pos() - key_len);
            }
        }
        None
    }

    pub fn pos(&self) -> usize {
        self.absolute_index
    }

    pub fn step_back(&mut self) {
        if self.absolute_index == 0 {
            return;
        }

        if self.index != 0 {
            self.index -= 1;
            self.absolute_index -= 1;
            return;
        }

        loop {
            if self.index == 0 {
                if self.vec == 0 {
                    return;
                }

                self.vec -= 1;
                self.index = self.rope.dna[self.vec].len();

                if self.index != 0 {
                    self.index -= 1;
                    self.absolute_index -= 1;
                    return;
                }
            }
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a DNA;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vec == self.rope.dna.len() {
            return None;
        }

        let dna = loop {
            let dna = &self.rope.dna[self.vec];
            if dna.len() == self.index {
                self.vec += 1;
                self.index = 0;
                if self.vec == self.rope.dna.len() {
                    return None;
                }
            } else {
                break dna;
            }
        };

        if self.index == dna.len() {
            return None;
        }

        let result = dna.get(self.index);
        self.index += 1;
        self.absolute_index += 1;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::DnaRope;
    use super::DNA::{self, *};

    fn sample_dna() -> DnaRope {
        DnaRope::from_raw(
            vec![
                vec![I, C, F, P],
                vec![],
                vec![I],
                vec![C],
                vec![F],
                vec![P],
                vec![I, I, I],
                vec![],
                vec![P, P],
                vec![F],
                vec![P, P],
                vec![],
                vec![P, P],
            ].into(),
        )
    }

    #[test]
    fn next() {
        let dna_rope = sample_dna();

        let mut iter = dna_rope.iter();

        assert_eq!(iter.next(), Some(&I));
        assert_eq!(iter.next(), Some(&C));
        assert_eq!(iter.next(), Some(&F));
        assert_eq!(iter.next(), Some(&P));
        assert_eq!(iter.next(), Some(&I));
        assert_eq!(iter.next(), Some(&C));
        assert_eq!(iter.next(), Some(&F));
        assert_eq!(iter.next(), Some(&P));
        assert_eq!(iter.next(), Some(&I));
        assert_eq!(iter.next(), Some(&I));
        assert_eq!(iter.next(), Some(&I));
        assert_eq!(iter.next(), Some(&P));
        assert_eq!(iter.next(), Some(&P));
        assert_eq!(iter.next(), Some(&F));
        assert_eq!(iter.next(), Some(&P));
        assert_eq!(iter.next(), Some(&P));
        assert_eq!(iter.next(), Some(&P));
        assert_eq!(iter.next(), Some(&P));
        assert_eq!(iter.next(), None);
        iter.step_back();
        assert_eq!(iter.next(), Some(&P));
        assert_eq!(iter.next(), None);

        let as_vec: Vec<&DNA> = dna_rope.as_vec();
        assert_eq!(
            as_vec,
            vec![&I, &C, &F, &P, &I, &C, &F, &P, &I, &I, &I, &P, &P, &F, &P, &P, &P, &P]
        );

        (0..as_vec.len()).for_each(|_| iter.step_back());
        assert_eq!(iter.next(), Some(&I));
        assert_eq!(iter.next(), Some(&C));
        assert_eq!(iter.next(), Some(&F));
        assert_eq!(iter.next(), Some(&P));
        iter.step_back();
        iter.step_back();
        assert_eq!(iter.next(), Some(&F));
    }

    fn check_index_pair(dna_rope: &DnaRope) {
        let data = (0..dna_rope.iter().count())
            .map(|x| match dna_rope.index_pair(x) {
                Some((vec, index)) => &dna_rope.dna[vec][index],
                None => panic!(),
            }).collect::<Vec<_>>();

        assert_eq!(data, dna_rope.iter().collect::<Vec<_>>());
    }

    #[test]
    fn index_pair() {
        let dna_rope = DnaRope::from(vec![]);
        assert_eq!(dna_rope.index_pair(0), None);

        let dna_rope = DnaRope::from(vec![I]);
        assert_eq!(dna_rope.index_pair(1), None);

        let dna_rope = DnaRope::from_raw(vec![vec![], vec![], vec![], vec![F]].into());
        assert_eq!(dna_rope.dna[dna_rope.index_pair(0).unwrap().0][0], F);

        let dna_rope = sample_dna();
        check_index_pair(&dna_rope);
        check_index_pair(&DnaRope::from_raw(
            vec![
                vec![C, I, F, F, F],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![I],
                vec![],
                vec![],
                vec![],
                vec![P, P, I, I, C, C, F, F],
                vec![],
            ].into(),
        ));
    }

    fn sample_three_group(a: usize, b: usize, c: usize) -> Vec<Vec<DNA>> {
        vec![vec![I; a], vec![C; b], vec![P; c]]
    }

    fn sample_three_group_flat(a: usize, b: usize, c: usize) -> Vec<DNA> {
        vec![I; a]
            .into_iter()
            .chain(vec![C; b].into_iter())
            .chain(vec![P; c].into_iter())
            .collect()
    }

    fn sample_three_group_dna(a: usize, b: usize, c: usize) -> DnaRope {
        DnaRope::from_raw(sample_three_group(a, b, c).into())
    }

    #[test]
    fn split_off() {
        let len = 10 + 20 + 30;
        for at in 0..=len {
            let arr = sample_three_group(10, 20, 30);
            let mut prefix: Vec<&DNA> = arr[0]
                .iter()
                .chain(arr[1].iter())
                .chain(arr[2].iter())
                .collect();
            let suffix = prefix.split_off(at);
            let mut dna_rope_prefix = sample_three_group_dna(10, 20, 30);
            let dna_rope_suffix = dna_rope_prefix.split_off(at);

            assert_eq!(dna_rope_prefix.dna.len(), dna_rope_prefix.index.len());
            if !dna_rope_prefix.index.is_empty() {
                assert_eq!(dna_rope_prefix.len(), prefix.len());
            }

            let dna_rope_prefix = dna_rope_prefix.as_vec();
            assert_eq!(dna_rope_prefix, prefix);

            let dna_rope_suffix = dna_rope_suffix.as_vec();
            assert_eq!(dna_rope_suffix, suffix);
        }
    }

    #[test]
    fn copy_from_range() {
        let source = sample_three_group_flat(10, 20, 30);
        let arr: Vec<_> = source.iter().collect();
        let dna_rope = sample_three_group_dna(10, 20, 30);
        for i in 0..arr.len() {
            for j in i..=arr.len() {
                let expected = &arr[i..j];
                let actual = &dna_rope.iter_from_range(i..j).collect::<Vec<_>>()[..];
                assert_eq!(actual, expected);
            }
        }
    }

    fn case_split_by_ranges(ranges: &[std::ops::Range<usize>]) {
        let source = sample_three_group_flat(10, 20, 30);
        let arr: Vec<_> = source.iter().collect();
        let dna_rope = sample_three_group_dna(10, 20, 30);
        let result = dna_rope.split_by_ranges(ranges);
        for (index, range) in ranges.iter().enumerate() {
            let expected = &arr[range.clone()];
            let actual: Vec<_> = result[index].as_vec();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn split_by_ranges() {
        case_split_by_ranges(&[0..10 + 20 + 30]);
        case_split_by_ranges(&[0..10, 0..10 + 20 + 30]);
        case_split_by_ranges(&[0..10, 10..10 + 20, 10 + 20..10 + 20 + 30]);
        case_split_by_ranges(&[5..15, 15..10 + 25, 10 + 25..10 + 20 + 29]);
        case_split_by_ranges(&[5..16, 15..10 + 24, 10 + 25..10 + 20 + 29]);

        let dna_rope = DnaRope::from(vec![I, C, F, P, F, F, F, F, C, C, C, P]);
        let result = dna_rope.split_by_ranges(&[0..4, 4..8, 0..4]);
        let mut dna_rope = DnaRope::from(vec![C, C, C, C]);
        for e in result {
            dna_rope.prepend(e);
        }
        let expected = vec![I, C, F, P, F, F, F, F, I, C, F, P, C, C, C, C];
        let actual: Vec<_> = dna_rope.iter().map(DNA::clone).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn iter_dna_search() {
        let dna_rope = DnaRope::from(vec![I, C, F, P]);
        let iter = dna_rope.iter();
        let old_pos = iter.pos();
        assert_eq!(iter.dna_search(&[I, C, F, P]), Some(0));
        assert_eq!(old_pos, iter.pos());

        let dna_rope = DnaRope::from(vec![I, C, I, C, I, C, I, C, I, C, F, I, C, I, C]);
        assert_eq!(dna_rope.iter().dna_search(&[I, C, F]), Some(8));
    }
}
