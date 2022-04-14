use super::{builders::Builder, IndexMode};
use crate::SuffixArray;
use std::{collections::BTreeMap, marker::PhantomData};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NaiveBuilder;

impl<B, T, Im> Builder<B, T, Im> for NaiveBuilder
where
    T: Ord,
    B: AsRef<[T]>,
    Im: IndexMode<T>,
{
    #[inline]
    fn build(values: B, mode: Im) -> crate::SuffixArray<B, T, Im> {
        SuffixArray::new_naive(values, mode)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BucketBuilder;

impl<B, T, Im> Builder<B, T, Im> for BucketBuilder
where
    T: Ord,
    B: AsRef<[T]>,
    Im: IndexMode<T>,
{
    #[inline]
    fn build(values: B, mode: Im) -> crate::SuffixArray<B, T, Im> {
        SuffixArray::new_bucket(values, mode)
    }
}

impl<T, B, Im> SuffixArray<B, T, Im>
where
    T: Ord,
    B: AsRef<[T]>,
    Im: IndexMode<T>,
{
    #[inline]
    pub(crate) fn new_zero_sized(values: B, mode: Im) -> Self {
        Self {
            values,
            indices: vec![],
            mode,
            value_type: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn sort_indices(values: &[T], indices: &mut [usize]) {
        indices.sort_by_key(|x| &values[*x..]);
    }

    pub(crate) fn new_naive(values: B, mode: Im) -> Self {
        let mut indices = values
            .as_ref()
            .iter()
            .enumerate()
            .filter_map(|(index, value)| mode.is_index(index, value).then(|| index))
            .collect::<Vec<_>>();
        Self::sort_indices(values.as_ref(), &mut indices);
        Self {
            values,
            indices,
            mode,
            value_type: PhantomData,
        }
    }

    pub(crate) fn new_bucket(values: B, mode: Im) -> Self {
        let source = values.as_ref();
        let mut indices = Vec::with_capacity(source.len());
        let mut tree = BTreeMap::new();
        for (i, v) in source.iter().enumerate() {
            if mode.is_index(i, v) {
                tree.entry(v).or_insert_with(Vec::new).push(i);
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

    #[cfg(feature = "gen_check")]
    pub(crate) fn gen_check(values: &[T], indices: &[usize]) {
        assert!(
            indices.iter().all(|x| *x != usize::MAX),
            "Invalid tmp index"
        );
        indices
            .iter()
            .enumerate()
            .try_fold(None, |o, (i, x)| {
                if let Some(o) = o {
                    if values[o..] < values[*x..] {
                        Ok(Some(*x))
                    } else {
                        Err((i, *x))
                    }
                } else {
                    Ok(Some(*x))
                }
            })
            .expect("not sorted propery");
    }

    #[cfg(not(feature = "gen_check"))]
    #[inline]
    pub(crate) fn gen_check(_values: &[T], _indices: &[usize]) {}

    pub(crate) fn check_remove_index(values: &[T], indices: &mut Vec<usize>, mode: &Im) {
        if mode.need_check() {
            Self::check_remove_index_inner(indices, mode, values);
        }
    }

    fn check_remove_index_inner(indices: &mut Vec<usize>, mode: &Im, values: &[T]) {
        for i in (0..indices.len()).rev() {
            let index = indices[i];
            if !mode.is_index(index, &values[index]) {
                indices.remove(i);
            }
        }
    }
}
