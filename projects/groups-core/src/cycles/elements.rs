use super::*;


impl CycleElement {
    pub fn length(&self) -> usize {
        self.chain.len()
    }
    /// Permute the data by permutations
    pub fn apply<T: Clone>(&self, data: &[T]) -> Vec<T> {
        let mut result = data.to_vec();
        unsafe { self.apply_in_place(&mut result) }
        result
    }
    /// Permute the data by permutations in-place
    pub unsafe fn apply_in_place<T>(&self, data: &mut [T]) {
        debug_assert!(self.chain.len() > 1, "Invalid Empty Cycle");
        let mut saved = None;
        for i in 0..self.length() {
            let current = *self.chain.get_unchecked(i);
            let next = *self.chain.get_unchecked((i + 1) % self.length());
            match saved {
                Some(saved_value) => data.swap(saved_value, current),
                None => saved = Some(current),
            }
            if self.chain.len() == i + 1 {
                data.swap(saved.unwrap_unchecked(), next);
            }
        }
    }
}
