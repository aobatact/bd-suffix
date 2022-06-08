//! module that provides some options to build [`SuffixArray`](`crate::SuffixArray`),
//! at [`new`](`crate::SuffixArray::new`).

pub mod modes;
mod sais;
mod simple;
mod two_stage;

pub(crate) use modes::*;

/// set of builders
/// pick one builder and use at [`new`](`crate::SuffixArray::new`)
/// recomment to use [`SAISBuilder`](`sais::SAISBuilder`) or [`SAISBuilderU8`](`sais::SAISBuilderU8`)
pub mod builders {
    impl<Buf, T, Im> SuffixArray<Buf, T, Im>
    where
        Buf: std::convert::AsRef<[T]>,
        T: std::cmp::Ord,
        Im: super::IndexMode<T> + Default,
    {
        /// Create new [`SuffixArray`] without selecting a `Builder`.
        pub fn new(values: Buf) -> Self {
            Self::new_by::<SAISBuilder>(values, Im::default())
        }
    }

    impl<Buf, T, Im> SuffixArray<Buf, T, Im> {
        /// Create new [`SuffixArray`] by using some algorythm selected by [`Builder`].
        pub fn new_by<B>(values: Buf, mode: Im) -> Self
        where
            B: Builder<Buf, T, Im>,
        {
            B::build(values, mode)
        }
    }

    /// Build a SuffixArray
    pub trait Builder<B, T, Im> {
        /// Create a new [`SuffixArray`]
        fn build(values: B, mode: Im) -> SuffixArray<B, T, Im>;
    }

    use crate::SuffixArray;

    pub use super::sais::*;
    pub use super::simple::*;
    pub use super::two_stage::*;
}

#[cfg(test)]
mod tests;
