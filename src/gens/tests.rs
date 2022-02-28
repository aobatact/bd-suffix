use crate::{
    gens::modes::{IndexMode, StrIndex},
    SuffixArray,
};

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
fn twostage_u8_g() {
    gen_test_base((), SuffixArray::new_two_stage);
}

// #[test]
// fn twostage_str_g() {
//     gen_test_base(StrIndex, SuffixArray::new_two_stage);
// }

// #[test]
// fn twostage_u8_s() {
//     gen_test_base((), SuffixArray::new_two_stage_u8);
// }

// #[test]
// fn twostage_str_s() {
//     gen_test_base(StrIndex, SuffixArray::new_two_stage_u8);
// }

#[test]
fn sais_u8_g() {
    gen_test_base((), SuffixArray::new_sais);
}

// #[test]
// fn sais_str_g() {
//     gen_test_base(StrIndex, SuffixArray::new_sais);
// }
