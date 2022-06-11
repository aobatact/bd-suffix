use std::marker::PhantomData;
pub mod gens;
mod search;

/// Simple suffix array
pub struct SuffixArray<B, T = u8, M = ()> {
    values: B,
    indices: Vec<usize>,
    mode: M,
    value_type: PhantomData<T>,
}

impl<T, B, M> SuffixArray<B, T, M>
where
    T: Ord,
    B: AsRef<[T]>,
{
    /// Get a reference to the suffix array's mode.
    pub fn mode(&self) -> &M {
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
