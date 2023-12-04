use std::str;

pub struct Lines<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> Lines<'a> {
    pub fn new(str: &'a str) -> Self {
        // strip any trailing '\n'
        let mut bytes = str.as_bytes();
        while bytes[bytes.len() - 1] == b'\n' {
            bytes = &bytes[..bytes.len() - 1];
        }

        Self { bytes, index: 0 }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.index;
        let mut end = start;

        if start == self.bytes.len() {
            return None;
        }

        loop {
            if let Some(&b) = self.bytes.get(end) {
                if b == b'\n' {
                    self.index = end + 1;
                    break;
                }
            } else {
                self.index = end;
                break;
            }
            end += 1;
        }

        let sl = &self.bytes[start..end];

        debug_assert!(str::from_utf8(sl).is_ok());

        Some(unsafe { str::from_utf8_unchecked(sl) })
    }
}

pub trait StrExt {
    fn iter_lines(&self) -> Lines<'_>;
}

impl<'a> StrExt for &'a str {
    fn iter_lines(&self) -> Lines<'_> {
        Lines::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Lines;

    #[test]
    fn test_line_iter() {
        let mut iter = Lines::new("abc\n123\n\ndef\n\n\n");

        assert_eq!(iter.next(), Some("abc"));
        assert_eq!(iter.next(), Some("123"));
        assert_eq!(iter.next(), Some(""));
        assert_eq!(iter.next(), Some("def"));
        assert_eq!(iter.next(), None);
    }
}
