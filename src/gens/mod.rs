pub mod modes;
mod sais;
mod simple;
mod two_stage;

pub(crate) use modes::*;

pub mod builders {
    impl<Buf, T, Im> SuffixArray<Buf, T, Im> {
        pub fn new<B>(values: Buf, mode: Im) -> Self
        where
            B: Builder<Buf, T, Im>,
        {
            B::build(values, mode)
        }
    }

    pub trait Builder<B, T, Im> {
        fn build(values: B, mode: Im) -> crate::SuffixArray<B, T, Im>;
    }

    use crate::SuffixArray;

    pub use super::sais::*;
    pub use super::simple::*;
    pub use super::two_stage::*;
}

#[cfg(test)]
mod tests;
