use std::marker::PhantomData;

use crate::{IndexMode, SuffixArray};

impl<T: Ord, B: AsRef<[T]>, Im: IndexMode<T>> SuffixArray<B, T, Im> {
    pub fn new_naive(values: B, mode: Im) -> Self {
        let mut indices = values
            .as_ref()
            .iter()
            .enumerate()
            .filter_map(|(index, value)| mode.is_index(index, value).then(|| index))
            .collect::<Vec<_>>();
        indices.sort_by_key(|x| &values.as_ref()[*x..]);
        Self {
            values,
            indices,
            mode,
            value_type: PhantomData,
        }
    }

    pub fn new_bucket(values: B, mode: Im) -> Self {
        use std::collections::BTreeMap;
        let source = values.as_ref();
        let mut indices = Vec::with_capacity(source.len());
        let mut tree = BTreeMap::new();
        for (i, v) in source.iter().enumerate() {
            if mode.is_index(i, v) {
                tree.entry(v).or_insert_with(|| vec![]).push(i);
            }
        }
        for (_, mut k_indices) in tree {
            k_indices.sort_by_key(|x| &source[*x..]);
            indices.append(&mut k_indices);
        }

        Self {
            values,
            indices,
            mode,
            value_type: PhantomData,
        }
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
        let suffix = f("abcde錆ac", mode);
        assert_eq!(suffix.search_naive("ab"), Ok(0));
        assert_eq!(suffix.search_naive("ac"), Ok(1));
        assert_eq!(suffix.search_naive("cb"), Err(4));
        M::test(suffix)
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
}
