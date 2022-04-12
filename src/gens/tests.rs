use crate::{
    gens::{IndexMode, StrIndex},
    SuffixArray,
};

fn gen_test_base<M, F, C>(mode: M, f: F, value: &'static str, cases: C)
where
    F: FnOnce(&'static str, M) -> SuffixArray<&'static str, u8, M>,
    M: ModeTester,
    C: IntoIterator<Item = (&'static str, Result<usize, usize>)>,
{
    let suffix = f(value, mode);
    for case in cases {
        assert_eq!(suffix.search_naive(case.0), case.1);
    }
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

fn gen_test_cases_u8<F>(f: F)
where
    F: FnOnce(&'static str, ()) -> SuffixArray<&'static str, u8, ()>,
{
    gen_test_base(
        (),
        f,
        "abcde錆さびacad",
        [
            ("ab", Ok(0)),
            ("abc", Ok(0)),
            ("abd", Err(1)),
            ("ac", Ok(1)),
            ("ba", Err(3)),
            ("bc", Ok(3)),
            ("bd", Err(4)),
            ("さび", Ok(15)),
            ("錆", Ok(17)),
        ],
    )
}

fn gen_test_cases_str<F>(f: F)
where
    F: FnOnce(&'static str, StrIndex) -> SuffixArray<&'static str, u8, StrIndex>,
{
    gen_test_base(
        StrIndex,
        f,
        "abcde錆さびacad",
        [
            ("ab", Ok(0)),
            ("abc", Ok(0)),
            ("abd", Err(1)),
            ("ac", Ok(1)),
            ("ba", Err(3)),
            ("bc", Ok(3)),
            ("bd", Err(4)),
            ("さび", Ok(9)),
        ],
    )
}

#[test]
fn naive_u8() {
    gen_test_cases_u8(SuffixArray::new_naive);
}

#[test]
fn naive_str() {
    gen_test_cases_str(SuffixArray::new_naive);
}

#[test]
fn bucket_u8() {
    gen_test_cases_u8(SuffixArray::new_bucket);
}

#[test]
fn bucket_str() {
    gen_test_cases_str(SuffixArray::new_bucket);
}

#[test]
fn twostage_u8_g() {
    gen_test_cases_u8(SuffixArray::new_two_stage);
}

#[test]
fn twostage_str_g() {
    gen_test_cases_str(SuffixArray::new_two_stage);
}

#[test]
fn twostage_u8_s() {
    gen_test_cases_u8(SuffixArray::new_two_stage_u8);
}

#[test]
fn twostage_str_s() {
    gen_test_cases_str(SuffixArray::new_two_stage_u8);
}

#[test]
fn sais_u8_g() {
    gen_test_cases_u8(SuffixArray::new_sais);
}

#[test]
fn sais_str_g() {
    gen_test_cases_str(SuffixArray::new_sais);
}
