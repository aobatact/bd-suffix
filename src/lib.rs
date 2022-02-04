use std::collections::BTreeMap;

pub struct SuffixArray<'a> {
    values: &'a [u8],
    indices: Vec<usize>,
}

impl<'a> SuffixArray<'a> {
    pub fn search_naive(&self, values: &[u8]) -> Result<usize, usize> {
        self.indices
            .binary_search_by_key(&values, |i| &self.values[*i..])
    }
}

impl<'a> SuffixArray<'a> {
    pub fn new_naive(values: &'a [u8]) -> Self {
        let mut indices = (0..values.len()).collect::<Vec<_>>();
        indices.sort_by_key(|x| &values[*x..]);
        Self { values, indices }
    }

    pub fn new_bucket(values: &'a [u8]) -> Self {
        let mut indices = Vec::with_capacity(values.len());
        let mut tree = BTreeMap::new();
        for (i, v) in values.iter().enumerate() {
            tree.entry(*v).or_insert_with(|| vec![]).push(i);
        }
        for (_, mut k_indices) in tree {
            k_indices.sort_by_key(|x| &values[*x..]);
            indices.append(&mut k_indices);
        }

        Self { values, indices }
    }

    // pub fn new_doubling_sort(values: &'a [u8]) -> Self {
    //     let mut indices = Vec::with_capacity(values.len());

    //     Self { values, indices }
    // }

    
    pub fn new_two_stage(values: &'a [u8]) -> Self {
        let indices =  todo!();
        Self { values, indices }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
