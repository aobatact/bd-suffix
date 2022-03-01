use super::modes::IndexMode;
use crate::SuffixArray;
use std::{collections::BTreeMap, marker::PhantomData, ops::AddAssign};

impl<T, B, Im> SuffixArray<B, T, Im>
where
    T: Ord,
    B: AsRef<[T]>,
    Im: IndexMode<T>,
{
    pub fn new_two_stage(values: B, mode: Im) -> Self
    where
        T: core::hash::Hash,
    {
        use bitvec::prelude::*;
        let source = values.as_ref();
        assert_ne!(source.len(), usize::MAX);
        let mut iter = source.iter().enumerate().rev();
        let mut last_v = None;
        let mut x = 0;
        //get the last item
        for (i, v) in &mut iter {
            if mode.is_index(i, v) {
                last_v = Some(v);
                x = i;
                break;
            }
        }
        let last_v = if let Some(last_v) = last_v {
            last_v
        } else {
            //zero index item -> early return
            return Self::new_zero_sized(values, mode);
        };
        let mut ltypes = BitVec::<usize, Lsb0>::repeat(false, x + 1);
        // ltypes.set(x, true); ignored because we know that it will be used bellow.
        let mut buckets = BTreeMap::new();
        let mut prev = last_v;
        // the last item is l type.
        buckets.insert(prev, (1, vec![]));
        let mut ind_count = 0;
        // Check whether the item is l or s type and throw it in the bucket.
        for (i, v) in iter {
            if mode.is_index(i, v) {
                ind_count += 1;
                let mut bucket = buckets.entry(v).or_insert_with(|| (0, vec![]));
                if prev < v {
                    ltypes.set(i, true);
                    bucket.0 += 1;
                } else {
                    bucket.1.push(i);
                }
                prev = v;
            }
        }
        let mut indices = Vec::with_capacity(ind_count);
        //l item counter
        let mut l_count_all = 0;
        for (_k, (ref mut l_count, ref mut s_indices)) in buckets.iter_mut() {
            let old_len = indices.len();
            // fill the l type slots at dummy slot.
            indices.extend(core::iter::repeat(usize::MAX).take(*l_count));
            l_count_all += *l_count;
            *l_count = old_len;
            // sort the s type and put in the slots.
            Self::sort_indices(&source, s_indices);
            indices.append(s_indices);
        }
        // the last item is l type and should be inserted here.
        unsafe {
            let h = &mut buckets.get_mut(last_v).unwrap_unchecked().0;
            *indices.get_unchecked_mut(*h) = x;
            *h += 1;
        }
        l_count_all -= 1;
        // fill the l types
        for i in 0..indices.len() {
            let ind = indices[i];
            if ind == 0 {
                continue;
            }
            if ind != usize::MAX {
                let mut ind_i = ind - 1;
                if mode.need_check() {
                    while !mode.is_index(ind_i, &source[ind_i]) {
                        ind_i -= 1;
                    }
                }
                let b_ref = unsafe { ltypes.get_unchecked(ind_i) };
                if b_ref == true {
                    unsafe {
                        let v = &source[ind_i];
                        let h = &mut buckets.get_mut(v).unwrap_unchecked().0;
                        indices[*h] = ind_i;
                        h.add_assign(1);
                    }
                    l_count_all -= 1;
                    if l_count_all == 0 {
                        break;
                    }
                }
            }
        }
        #[cfg(test)]
        {
            Self::gen_check(source, &indices)
        }
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
    pub fn new_two_stage_u8(values: B, mode: Im) -> Self {
        use bitvec::prelude::*;
        let source = values.as_ref();
        assert_ne!(source.len(), usize::MAX);
        let mut iter = source.iter().enumerate().rev();
        let mut last_v = None;
        let mut x = 0;
        //get the last item
        for (i, v) in &mut iter {
            if mode.is_index(i, v) {
                last_v = Some(v);
                x = i;
                break;
            }
        }
        let last_v = if let Some(last_v) = last_v {
            last_v
        } else {
            //zero index item -> early return
            return Self::new_zero_sized(values, mode);
        };
        let mut ltypes = BitVec::<usize, Lsb0>::repeat(false, x + 1);
        // ltypes.set(x, true); ignored because we know that it will be used bellow.
        let mut buckets = [(); 256].map(|_| (0, vec![]));
        let mut prev = last_v;
        // the last item is l type.
        buckets[*prev as usize].0 = 1;
        let mut ind_count = 0;
        // Check whether the item is l or s type and throw it in the bucket.
        for (i, v) in iter {
            if mode.is_index(i, v) {
                ind_count += 1;
                let mut bucket = &mut buckets[*v as usize];
                if prev < v {
                    ltypes.set(i, true);
                    bucket.0 += 1;
                } else {
                    bucket.1.push(i);
                }
                prev = v;
            }
        }
        let mut indices = Vec::with_capacity(ind_count);
        //l item counter
        let mut l_count_all = 0;
        for (_k, (ref mut l_count, ref mut s_indices)) in buckets.iter_mut().enumerate() {
            let old_len = indices.len();
            // fill the l type slots at dummy slot.
            indices.extend(core::iter::repeat(usize::MAX).take(*l_count));
            l_count_all += *l_count;
            *l_count = old_len;
            // sort the s type and put in the slots.
            Self::sort_indices(&source, s_indices);
            indices.append(s_indices);
        }
        // the last item is l type and should be inserted here.
        unsafe {
            let h = &mut buckets.get_mut(*last_v as usize).unwrap_unchecked().0;
            *indices.get_unchecked_mut(*h) = x;
            *h += 1;
        }
        l_count_all -= 1;
        // fill the l types
        for i in 0..indices.len() {
            let ind = indices[i];
            if ind == 0 {
                continue;
            }
            if ind != usize::MAX {
                let mut ind_i = ind - 1;
                while !mode.is_index(ind_i, &source[ind_i]) {
                    ind_i -= 1;
                }
                let b_ref = unsafe { ltypes.get_unchecked(ind_i) };
                if b_ref == true {
                    unsafe {
                        let v = &source[ind_i];
                        let h = &mut buckets.get_mut(*v as usize).unwrap_unchecked().0;
                        indices[*h] = ind_i;
                        h.add_assign(1);
                    }
                    l_count_all -= 1;
                    if l_count_all == 0 {
                        break;
                    }
                }
            }
        }
        Self::gen_check(source, &indices);
        Self {
            values,
            indices,
            mode,
            value_type: PhantomData,
        }
    }
}
