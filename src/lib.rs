use std::marker::PhantomData;

mod gen_simple;

pub struct SuffixArray<B, T = u8, Im = ()> {
    values: B,
    indices: Vec<usize>,
    mode: Im,
    value_type: PhantomData<T>,
}

impl<T : Ord, B: AsRef<[T]>, Im> SuffixArray<B, T, Im> {
    pub fn search_naive<B2>(&self, values: B2) -> Result<usize, usize>
    where
        B2: AsRef<[T]>,
    {
        let target = values.as_ref();
        let tlen = target.len();
        let source = self.values.as_ref();
        let slen = source.len();
        self.indices.binary_search_by_key(&target, |i| {
            let i = *i;
            if i + tlen < slen {
                &source[i..i + tlen]
            } else {
                &source[i..]
            }
        })
    }

    /// Get a reference to the suffix array's mode.
    pub fn mode(&self) -> &Im {
        &self.mode
    }
}

pub trait IndexMode<T> {
    fn is_index(&self, pos: usize, value: &T) -> bool;
    #[inline]
    fn need_check(&self) -> bool {
        true
    }
}

impl<T> IndexMode<T> for () {
    #[inline]
    fn is_index(&self, _pos: usize, _value: &T) -> bool {
        true
    }
    #[inline]
    fn need_check(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StrIndex;
impl IndexMode<u8> for StrIndex {
    fn is_index(&self, _pos: usize, value: &u8) -> bool {
        (*value as i8) >= -0x40
    }
}
