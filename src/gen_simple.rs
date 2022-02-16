use std::{collections::BTreeMap, fmt::Debug, marker::PhantomData, ops::AddAssign};

use crate::{IndexMode, SuffixArray};

impl<T, B, Im> SuffixArray<B, T, Im>
where
    T: Ord,
    B: AsRef<[T]>,
    Im: IndexMode<T>,
{
    #[inline]
    fn sort_indices(values: &[T], indices: &mut [usize]) {
        indices.sort_by_key(|x| &values[*x..]);
    }

    pub fn new_naive(values: B, mode: Im) -> Self {
        let mut indices = values
            .as_ref()
            .iter()
            .enumerate()
            .filter_map(|(index, value)| mode.is_index(index, value).then(|| index))
            .collect::<Vec<_>>();
        Self::sort_indices(&values.as_ref(), &mut indices);
        Self {
            values,
            indices,
            mode,
            value_type: PhantomData,
        }
    }

    pub fn new_bucket(values: B, mode: Im) -> Self {
        let source = values.as_ref();
        let mut indices = Vec::with_capacity(source.len());
        let mut tree = BTreeMap::new();
        for (i, v) in source.iter().enumerate() {
            if mode.is_index(i, v) {
                tree.entry(v).or_insert_with(|| vec![]).push(i);
            }
        }
        for (_, mut k_indices) in tree {
            Self::sort_indices(&source, &mut k_indices);
            indices.append(&mut k_indices);
        }

        Self {
            values,
            indices,
            mode,
            value_type: PhantomData,
        }
    }

    pub fn new_two_stage(values: B, mode: Im) -> Self
    where
        T: core::hash::Hash + Debug,
    {
        use bitvec::prelude::*;
        let source = values.as_ref();
        assert_ne!(source.len(), usize::MAX);
        let mut iter = source.iter().enumerate().rev();
        let mut last_v = None;
        let mut x = 0;
        for (i, v) in &mut iter {
            if mode.is_index(i, v) {
                last_v = Some(v);
                x = i;
                break;
            }
        }
        let last_v = if let Some(last_v) = last_v {
            last_v
        } else {
            return Self {
                values,
                indices: vec![0],
                mode,
                value_type: PhantomData,
            };
        };
        let mut ltypes = BitVec::<usize, Lsb0>::repeat(false, x + 1);
        // ltypes.set(x, true);
        let mut buckets = BTreeMap::new();
        let mut prev = last_v;
        buckets.insert(prev, (1, vec![]));
        let mut ind_count = 0;
        for (i, v) in iter {
            if mode.is_index(i, v) {
                ind_count += 1;
                let mut bucket = buckets.entry(v).or_insert_with(|| (0, vec![]));
                if prev > v {
                    ltypes.set(i, true);
                    bucket.0 += 1;
                } else {
                    bucket.1.push(i);
                }
                prev = v;
            }
        }
        let mut indices = Vec::with_capacity(ind_count);
        let mut l_count_all = 0;
        for (_k, (ref mut l_count, ref mut s_indices)) in buckets.iter_mut() {
            let old_len = indices.len();
            indices.extend(core::iter::repeat(usize::MAX).take(*l_count));
            l_count_all += *l_count;
            *l_count = old_len;
            Self::sort_indices(&source, s_indices);
            indices.append(s_indices);
        }
        unsafe {
            let h = &mut buckets.get_mut(last_v).unwrap_unchecked().0;
            *indices.get_unchecked_mut(*h) = x;
            *h += 1;
        }

        l_count_all -= 1;
        'outer: while l_count_all != 0 {
            for i in 0..indices.len() {
                let ind = indices[i];
                if ind == 0 {
                    continue;
                }
                if ind != usize::MAX {
                    let mut ind_i = ind - 1;
                    while !mode.is_index(ind_i, &source[ind_i]) {
                        ind_i -= 1;
                    }
                    let mut b_ref = unsafe { ltypes.get_unchecked_mut(ind_i) };
                    if b_ref == true {
                        b_ref.set(true);
                        unsafe {
                            let v = &source[ind_i];
                            let h = &mut buckets.get_mut(v).unwrap_unchecked().0;
                            indices[*h] = ind_i;
                            h.add_assign(1);
                        }
                        l_count_all -= 1;
                        if l_count_all == 0 {
                            break 'outer;
                        }
                    }
                }
            }
        }
        debug_assert!(Self::gen_test(source, &indices));

        Self {
            values,
            indices,
            mode,
            value_type: PhantomData,
        }
    }

    fn gen_test(values: &[T], indices: &[usize]) -> bool {
        indices.iter().all(|x| *x != usize::MAX)
            && indices
                .iter()
                .try_fold(None, |o, x| {
                    if let Some(o) = o {
                        if values[o] < values[*x] {
                            Some(Some(*x))
                        } else {
                            None
                        }
                    } else {
                        Some(Some(*x))
                    }
                })
                .is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::{IndexMode, StrIndex, SuffixArray};

    fn gen_test_base<M, F>(mode: M, f: F)
    where
        F: FnOnce(&'static str, M) -> SuffixArray<&'static str, u8, M>,
        M: ModeTester,
    {
        // let suffix = f("abac", mode);
        // assert_eq!(suffix.search_naive("ab"), Ok(0));
        // assert_eq!(suffix.search_naive("ba"), Ok(2));
        // M::test(suffix);
        let suffix = f("abcde錆acad", mode);
        assert_eq!(suffix.search_naive("ab"), Ok(0));
        assert_eq!(suffix.search_naive("ac"), Ok(1));
        assert!(suffix.search_naive("錆").is_ok());
        assert_eq!(suffix.search_naive("cb"), Err(5));
        M::test(suffix);
    }

    trait ModeTester: IndexMode<u8> + Sized {
        fn test(array: SuffixArray<&'static str, u8, Self>);
    }

    impl ModeTester for () {
        fn test(array: SuffixArray<&'static str, u8, Self>) {
            assert_eq!(array.indices.len(), array.values.bytes().len());
        }
    }
    impl ModeTester for StrIndex {
        fn test(array: SuffixArray<&'static str, u8, Self>) {
            assert_eq!(array.indices.len(), array.values.chars().count());
        }
    }

    #[test]
    fn naive_u8() {
        gen_test_base((), SuffixArray::new_naive);
    }

    #[test]
    fn naive_str() {
        gen_test_base(StrIndex, SuffixArray::new_naive);
    }

    #[test]
    fn bucket_u8() {
        gen_test_base((), SuffixArray::new_bucket);
    }

    #[test]
    fn bucket_str() {
        gen_test_base(StrIndex, SuffixArray::new_bucket);
    }

    #[test]
    fn twostage_u8() {
        gen_test_base((), SuffixArray::new_two_stage);
    }
}
