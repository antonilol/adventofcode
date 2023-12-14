pub trait StrExt {
    fn map_1d_vec<T, F: FnMut(u8) -> T>(&self, f: F) -> Vec<T>;

    fn map_2d_vec<T, F: FnMut(u8) -> T>(&self, f: F) -> Vec<Vec<T>>;
}

impl StrExt for str {
    fn map_1d_vec<T, F: FnMut(u8) -> T>(&self, mut f: F) -> Vec<T> {
        self.bytes().map(&mut f).collect()
    }

    fn map_2d_vec<T, F: FnMut(u8) -> T>(&self, mut f: F) -> Vec<Vec<T>> {
        self.lines()
            .map(|line| line.bytes().map(&mut f).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::ext::str::StrExt;

    #[test]
    fn test_map2d() {
        assert_eq!(
            "###\n#..\n#..".map_2d_vec(|b| match b {
                b'#' => true,
                b'.' => false,
                _ => unreachable!(),
            }),
            [
                [true, true, true],
                [true, false, false],
                [true, false, false]
            ]
        );
    }
}
