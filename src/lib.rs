use std::marker::PhantomData;
pub mod gens;

/// Simple suffix array
pub struct SuffixArray<B, T = u8, Im = ()> {
    values: B,
    indices: Vec<usize>,
    mode: Im,
    value_type: PhantomData<T>,
}

impl<T, B, Im> SuffixArray<B, T, Im>
where
    T: Ord,
    B: AsRef<[T]>,
{
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

    /// Get a reference to the suffix array's indices.
    pub fn indices(&self) -> &[usize] {
        self.indices.as_ref()
    }

    /// Get a reference to the suffix array's values.
    pub fn values(&self) -> &B {
        &self.values
    }
}
