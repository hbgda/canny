use std::fmt::Debug;

pub mod pattern {
    use std::ops::Deref;

    #[derive(Debug)]
    pub enum Part {
        Byte(u8),
        Skip
    }

    #[derive(Debug)]
    pub struct Pattern(Vec<Part>);

    impl Pattern {
        pub fn new(pattern: &str) -> Pattern {
            let parts: Vec<Part> = pattern.to_ascii_uppercase()
                .split(" ")
                .map(|c| 
                    match c { 
                        "??" => Part::Skip, 
                        _ => {
                            let mut bytes = c.bytes()
                                .filter(|f| f.is_ascii_alphanumeric())
                                .map(|mut f| { if f.is_ascii_alphabetic() { f -= 7 } f - 48})
                                .collect::<Vec<_>>();
                            if bytes.len() > 1 {
                                bytes[0] *= 16;
                            }
                            Part::Byte(
                                dbg!(bytes.iter().sum())
                            )
                        }
                    }
                )
                .collect();

            Pattern(parts)
        }
    }

    impl Deref for Pattern {
        type Target = Vec<Part>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

pub trait ByteStream: Iterator<Item = u8> + Clone {}
impl<T: Iterator<Item = u8> + Clone> ByteStream for T {}

#[derive(Debug)]
pub struct Scanner<T: ByteStream> {
    bytes: T,
    bytes_len: usize,
    pattern: pattern::Pattern,
    idx: usize
}

impl<T: ByteStream + Debug> Scanner<T> {
    pub fn new(bytes: T, pattern: pattern::Pattern) -> Scanner<T> {
        Scanner {
            bytes_len: bytes.clone().count(),
            bytes,
            pattern,
            idx: 0
        }
    }
}

impl<T: ByteStream> Iterator for Scanner<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > self.bytes_len - self.pattern.len() {
            return None;
        }
        for part in self.pattern.iter() {
            let pattern::Part::Byte(pattern_byte) = *part else { self.bytes.next(); continue };
            if dbg!(self.bytes.next()?) != dbg!(pattern_byte) { self.idx += 1; return self.next(); }
        }

        Some(self.idx)
    }
}

#[cfg(test)]
pub mod test {
    use crate::{Scanner, pattern::Pattern};

    #[test]
    fn test() {
        let bytes = vec![0x3, 0x12, 0x58, 0xFF, 0x0, 0x1, 0x2, 0x3];
        let mut scanner = Scanner::new(bytes.into_iter(), Pattern::new("FF ?? 01"));
        let found = scanner.next();
        assert_eq!(Some(3), found);
    }
}