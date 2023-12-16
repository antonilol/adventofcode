pub trait IntoIteratorExt: IntoIterator + Sized {
    fn fold_2d<T, F>(self, init: T, mut f: F) -> T
    where
        Self::Item: IntoIterator,
        F: FnMut(T, <Self::Item as IntoIterator>::Item, (usize, usize)) -> T,
    {
        self.into_iter().enumerate().fold(init, |acc, (y, it)| {
            it.into_iter()
                .enumerate()
                .fold(acc, |acc, (x, v)| f(acc, v, (x, y)))
        })
    }
}

impl<T: IntoIterator> IntoIteratorExt for T {}

#[cfg(test)]
mod tests {
    use super::IntoIteratorExt;

    #[test]
    fn test_fold_2d() {
        let arr = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
        let result = arr.fold_2d(true, |acc, item, (x, y)| acc && (arr[y][x] == item));
        assert!(result);
    }
}
