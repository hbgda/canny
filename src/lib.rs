use std::rc::Rc;

pub mod pattern {
    pub enum Part {
        Byte(u8),
        Skip
    }
    pub struct Pattern(Vec<Part>);

    impl Pattern {
        pub fn new(pattern: &str) -> Pattern {
            let parts: Vec<Part> = pattern
                .bytes()
                .map(|c| match c { b'?' => Part::Skip, _ => Part::Byte(c) })
                .collect();

            Pattern(parts)
        }
    }
}

pub trait ByteStream: Iterator<Item = u8> {}
impl<T: Iterator<Item = u8>> ByteStream for T {}

pub struct Scanner<T: ByteStream> {
    bytes: T,
    pattern: pattern::Pattern
}

impl<T: ByteStream> Scanner<T> {
    pub fn new(bytes: T, pattern: pattern::Pattern) -> Scanner<T> {
        Scanner {
            bytes,
            pattern
        }
    }
}

impl<T: ByteStream> Iterator for Scanner<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
pub mod test {
    use crate::{Scanner, pattern::Pattern};

    #[test]
    fn test() {
        let bytes = vec![0x0, 0x1, 0x2, 0x3];
        let mut scanner = Scanner::new(bytes.into_iter(), Pattern::new("\0x1\0x2"));
        assert_eq!(1usize, scanner.next().unwrap())
    }
}