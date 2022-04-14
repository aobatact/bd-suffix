use super::{builders::Builder, IndexMode};
use crate::SuffixArray;
use bitvec::prelude::*;
use std::{collections::BTreeMap, iter, marker::PhantomData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SAISBuilder;

impl<T, B, Im> Builder<B, T, Im> for SAISBuilder
where
    T: Ord,
    B: AsRef<[T]>,
    Im: IndexMode<T>,
{
    #[inline]
    fn build(values: B, mode: Im) -> crate::SuffixArray<B, T, Im> {
        SuffixArray::new_sais(values, mode)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SAISBuilderU8;

impl<B, Im> Builder<B, u8, Im> for SAISBuilderU8
where
    B: AsRef<[u8]>,
    Im: IndexMode<u8>,
{
    #[inline]
    fn build(values: B, mode: Im) -> crate::SuffixArray<B, u8, Im> {
        SuffixArray::new_sais_u8(values, mode)
    }
}

impl<T, B, Im> SuffixArray<B, T, Im>
where
    T: Ord,
    B: AsRef<[T]>,
    Im: IndexMode<T>,
{
    pub(crate) fn new_sais(values: B, mode: Im) -> Self {
        let source = values.as_ref();
        assert_ne!(source.len(), usize::MAX);
        let mut iter = source.iter().enumerate().rev();
        let mut last_v = None;
        let mut last_i = 0;
        //get the last item
        for (i, v) in &mut iter {
            // if true || mode.is_index(i, v) {
            {
                last_v = Some(v);
                last_i = i;
                break;
            }
        }
        let last_v = if let Some(last_v) = last_v {
            last_v
        } else {
            //zero index item -> early return
            return Self::new_zero_sized(values, mode);
        };
        let mut ltypes = BitVec::<usize, Lsb0>::repeat(false, last_i + 1);
        ltypes.set(last_i, true);
        let mut buckets = BTreeMap::new();
        buckets.insert(last_v, (1, 0, vec![]));
        let mut prev = last_v;
        let mut s_flag = false;
        let mut prev_s_i = last_i;
        let mut full_count = 1;
        for (i, v) in iter {
            // if mode.is_index(i, v) {
            {
                full_count += 1;
                let bucket = buckets.entry(v).or_insert_with(|| (0, 0, vec![]));
                if prev < v {
                    bucket.0 += 1;
                    ltypes.set(i, true);
                    if s_flag {
                        s_flag = false;
                        unsafe { buckets.get_mut(prev).unwrap_unchecked() }
                            .2
                            .push(prev_s_i);
                    }
                } else {
                    bucket.1 += 1;
                    prev_s_i = i;
                    s_flag = true;
                }
                prev = v;
            }
        }
        let mut indices = Vec::with_capacity(full_count);
        let mut counter = 0;
        for (_k, (lc, sc, lms)) in &mut buckets {
            indices.extend(iter::repeat(usize::MAX).take(*lc + *sc - lms.len()));
            *sc += *lc + counter;
            *lc = counter;
            counter = *sc;

            indices.extend(lms.drain(..).rev());
        }
        // the last item is l type and should be inserted here.
        ltypes.set(last_i, true);
        unsafe {
            let h = &mut buckets.get_mut(last_v).unwrap_unchecked().0;
            *indices.get_unchecked_mut(*h) = last_i;
            *h += 1;
        }
        for i in 0..full_count {
            let ind = indices[i];
            if ind == 0 {
                continue;
            }
            if ind != usize::MAX {
                let ind_i = ind - 1;
                // if mode.need_check() {
                //     while !mode.is_index(ind_i, &source[ind_i]) {
                //         ind_i -= 1;
                //     }
                // }
                let b_ref = unsafe { ltypes.get_unchecked(ind_i) };
                if b_ref == true {
                    unsafe {
                        let v = &source[ind_i];
                        let h = &mut buckets.get_mut(v).unwrap_unchecked().0;
                        indices[*h] = ind_i;
                        *h += 1;
                    }
                }
            }
        }
        for i in (0..full_count).rev() {
            let ind = indices[i];
            if ind == 0 {
                continue;
            }
            if unsafe { ltypes.get_unchecked(ind) } == true {
                let ind_i = ind - 1;
                // if mode.need_check() {
                //     while !mode.is_index(ind_i, &source[ind_i]) {
                //         ind_i -= 1;
                //     }
                // }
                let mut b_ref = unsafe { ltypes.get_unchecked_mut(ind_i) };
                if b_ref == false {
                    unsafe {
                        b_ref.set(true);
                        let v = &source[ind_i];
                        let h = &mut buckets.get_mut(v).unwrap_unchecked().1;
                        *h -= 1;
                        indices[*h] = ind_i;
                    }
                }
            }
        }
        Self::gen_check(source, &indices);
        Self::check_remove_index(source, &mut indices, &mode);
        Self {
            values,
            indices,
            mode,
            value_type: PhantomData,
        }
    }
}

impl<B, Im> SuffixArray<B, u8, Im>
where
    B: AsRef<[u8]>,
    Im: IndexMode<u8>,
{
    pub(crate) fn new_sais_u8(values: B, mode: Im) -> Self {
        let source = values.as_ref();
        assert_ne!(source.len(), usize::MAX);
        let mut iter = source.iter().enumerate().rev();
        let mut last_v = None;
        let mut last_i = 0;
        //get the last item
        for (i, v) in &mut iter {
            // if mode.is_index(i, v) {
            {
                last_v = Some(v);
                last_i = i;
                break;
            }
        }
        let last_v = if let Some(last_v) = last_v {
            last_v
        } else {
            //zero index item -> early return
            return Self::new_zero_sized(values, mode);
        };
        let mut ltypes = BitVec::<usize, Lsb0>::repeat(false, last_i + 1);
        ltypes.set(last_i, true);
        let mut buckets = [(); 256].map(|_| (0, 0, vec![]));
        buckets[*last_v as usize] = (1, 0, vec![]);
        let mut prev = last_v;
        let mut s_flag = false;
        let mut prev_s_i = last_i;
        let mut full_count = 1;
        for (i, v) in iter {
            // if mode.is_index(i, v) {
            {
                full_count += 1;
                let bucket = unsafe { buckets.get_unchecked_mut(*v as usize) };
                if prev < v {
                    bucket.0 += 1;
                    ltypes.set(i, true);
                    if s_flag {
                        s_flag = false;
                        unsafe { buckets.get_mut(*prev as usize).unwrap_unchecked() }
                            .2
                            .push(prev_s_i);
                    }
                } else {
                    bucket.1 += 1;
                    prev_s_i = i;
                    s_flag = true;
                }
                prev = v;
            }
        }
        let mut indices = Vec::with_capacity(full_count);
        let mut counter = 0;
        for (lc, sc, lms) in &mut buckets {
            indices.extend(iter::repeat(usize::MAX).take(*lc + *sc - lms.len()));
            *sc += *lc + counter;
            *lc = counter;
            counter = *sc;

            indices.extend(lms.drain(..).rev());
        }
        // the last item is l type and should be inserted here.
        ltypes.set(last_i, true);
        unsafe {
            let h = &mut buckets.get_mut(*last_v as usize).unwrap_unchecked().0;
            *indices.get_unchecked_mut(*h) = last_i;
            *h += 1;
        }
        for i in 0..full_count {
            let ind = indices[i];
            if ind == 0 {
                continue;
            }
            if ind != usize::MAX {
                let ind_i = ind - 1;
                // if mode.need_check() {
                //     while !mode.is_index(ind_i, &source[ind_i]) {
                //         ind_i -= 1;
                //     }
                // }
                let b_ref = unsafe { ltypes.get_unchecked(ind_i) };
                if b_ref == true {
                    unsafe {
                        let v = &source[ind_i];
                        let h = &mut buckets.get_mut(*v as usize).unwrap_unchecked().0;
                        indices[*h] = ind_i;
                        *h += 1;
                    }
                }
            }
        }
        for i in (0..full_count).rev() {
            let ind = indices[i];
            if ind == 0 {
                continue;
            }
            if unsafe { ltypes.get_unchecked(ind) } == true {
                let ind_i = ind - 1;
                // if mode.need_check() {
                //     while !mode.is_index(ind_i, &source[ind_i]) {
                //         ind_i -= 1;
                //     }
                // }
                let mut b_ref = unsafe { ltypes.get_unchecked_mut(ind_i) };
                if b_ref == false {
                    unsafe {
                        b_ref.set(true);
                        let v = &source[ind_i];
                        let h = &mut buckets.get_mut(*v as usize).unwrap_unchecked().1;
                        *h -= 1;
                        indices[*h] = ind_i;
                    }
                }
            }
        }
        Self::gen_check(source, &indices);
        Self::check_remove_index(source, &mut indices, &mode);
        Self {
            values,
            indices,
            mode,
            value_type: PhantomData,
        }
    }
}
