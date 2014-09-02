use super::*;

impl FromIterator<usize> for CycleElement {
    fn from_iter<I>(iter: I) -> Self where I: IntoIterator<Item=usize> {
        Self { chain: iter.into_iter().collect() }
    }
}
impl<'i> IntoIterator for &'i CycleNotation {
    type Item = (usize, usize);
    type IntoIter = impl Iterator<Item=(usize, usize)>;

    fn into_iter(self) -> Self::IntoIter {
         self.cycles.iter().flatten()
    }
}

impl<'i> IntoIterator for &'i CycleElement {
    type Item = (usize, usize);
    type IntoIter = impl Iterator<Item=(usize, usize)>;

    fn into_iter(self) -> Self::IntoIter {
        from_coroutine(move |yy: _| {
            if self.chain.len() > 1 {
                for i in 0..self.length() {
                    unsafe {
                        let current = *self.chain.get_unchecked(i);
                        let next = *self.chain.get_unchecked((i + 1) % self.length());
                        yield (current, next);
                    }
                }
            }
        })
    }
}