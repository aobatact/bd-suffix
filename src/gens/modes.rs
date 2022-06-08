//! `modes` is to select which index is used to search. See [`IndexMode::is_index`].

/// Index setting for building at [`SuffixArray::new`](`crate::SuffixArray::new`).
/// See [`is_index`](`IndexMode::is_index`).
pub trait IndexMode<T> {
    /// Check whether the index is used in search.
    ///
    /// Some values won't be used to search because of some reason,
    /// [`str`] (or utf8) has sequence boundaries and non boundaries places won't be used to index,
    /// so this will prevent these index from searching.
    fn is_index(&self, pos: usize, value: &T) -> bool;
    /// Returns true if the indice might have some invalid sequence or want to ignore checks.
    ///
    /// Should only return `true` if [`is_index`](`Self::is_index`) always returns true,
    /// and `()` can be used as [`IndexMode`] for that case.
    #[inline]
    fn need_check(&self) -> bool {
        true
    }
}

/// This IndexMode ignores all checks.
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

/// [`IndexMode`] for [`str`] or [`String`]. This removes index pointing on non Utf8 sequence boundary.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StrIndex;

impl IndexMode<u8> for StrIndex {
    #[inline]
    fn is_index(&self, _pos: usize, value: &u8) -> bool {
        (*value as i8) >= -0x40
    }
}
