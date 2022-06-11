use std::cmp::Ordering;

use crate::SuffixArray;

impl<T, B, M> SuffixArray<B, T, M>
where
    T: Ord,
    B: AsRef<[T]>,
{
    pub fn search<S: Searcher<B, T, M, B2>, B2>(
        &self,
        values: B2,
    ) -> Result<(usize, usize), usize> {
        S::search_range(self, values)
    }

    pub fn search_naive<B2>(&self, values: B2) -> Result<(usize, usize), usize>
    where
        B2: AsRef<[T]>,
    {
        self.search::<NaiveSearcher, _>(values)
    }
}

pub trait Searcher<B, T, M, B2> {
    fn search_range(sa: &SuffixArray<B, T, M>, target: B2) -> Result<(usize, usize), usize>;
    fn search_contains(sa: &SuffixArray<B, T, M>, target: B2) -> bool {
        Self::search_range(sa, target).is_ok()
    }
}

pub struct NaiveSearcher;

impl<B: AsRef<[T]>, T: Ord, M, B2: AsRef<[T]>> Searcher<B, T, M, B2> for NaiveSearcher {
    fn search_range(sa: &SuffixArray<B, T, M>, target: B2) -> Result<(usize, usize), usize> {
        let t = target.as_ref();
        let tlen = t.len();
        let vals = sa.values().as_ref();
        let len = vals.len();
        binary_search_range_by(sa.indices(), |i| {
            let i = *i;
            if i + tlen < len {
                t.cmp(&vals[i..i + tlen])
            } else {
                t.cmp(&vals[i..])
            }
        })
    }

    fn search_contains(sa: &SuffixArray<B, T, M>, target: B2) -> bool {
        let t = target.as_ref();
        let tlen = t.len();
        let vals = sa.values().as_ref();
        let len = vals.len();
        binary_first_match(sa.indices(), |i| {
            let i = *i;
            if i + tlen < len {
                t.cmp(&vals[i..i + tlen])
            } else {
                t.cmp(&vals[i..])
            }
        })
        .is_ok()
    }
}

fn binary_search_range_by<T, F>(array: &[T], mut f: F) -> Result<(usize, usize), usize>
where
    F: FnMut(&T) -> Ordering,
{
    let (mut l, mut r, first) = binary_first_match(array, &mut f)?;
    {
        let mut l_r = first;
        while l < l_r {
            let mid = (l + l_r) / 2;
            let cmp = f(&array[mid]);
            if cmp == Ordering::Greater {
                l = mid + 1;
            } else {
                l_r = mid;
            }
        }
    }
    {
        let mut r_l = first;
        while r_l < r {
            let mid = (r + r_l) / 2;
            let cmp = f(&array[mid]);
            if cmp == Ordering::Less {
                r = mid;
            } else {
                r_l = mid + 1;
            }
        }
    }
    Ok((l, r))
}

fn binary_first_match<T, F>(array: &[T], mut f: F) -> Result<(usize, usize, usize), usize>
where
    F: FnMut(&T) -> Ordering,
{
    let mut l = 0;
    let mut r = array.len();
    let first = loop {
        if l >= r {
            return Err(l);
        }
        let mid = (l + r) / 2;
        let cmp = f(&array[mid]);
        if cmp == Ordering::Less {
            r = mid;
        } else if cmp == Ordering::Greater {
            l = mid + 1;
        } else {
            break mid;
        }
    };
    Ok((l, r, first))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_search() {
        let x = [1, 3, 5, 5, 6, 7, 8, 10, 12, 12, 12];
        assert_eq!(super::binary_search_range_by(&x, |x| 1.cmp(x)), Ok((0, 1)));
        assert_eq!(super::binary_search_range_by(&x, |x| 2.cmp(x)), Err(1));
        assert_eq!(super::binary_search_range_by(&x, |x| 3.cmp(x)), Ok((1, 2)));
        assert_eq!(super::binary_search_range_by(&x, |x| 5.cmp(x)), Ok((2, 4)));
        assert_eq!(super::binary_search_range_by(&x, |x| 10.cmp(x)), Ok((7, 8)));
        assert_eq!(super::binary_search_range_by(&x, |x| 11.cmp(x)), Err(8));
        assert_eq!(super::binary_search_range_by(&x, |x| 12.cmp(x)), Ok((8, 11)));
    }
}
