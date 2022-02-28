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
    #[inline]
    fn is_index(&self, _pos: usize, value: &u8) -> bool {
        (*value as i8) >= -0x40
    }
}
