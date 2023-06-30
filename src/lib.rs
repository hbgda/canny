use std::rc::Rc;

pub mod pattern {
    pub enum Part {
        Byte(u8),
        Skip
    }
    pub struct Pattern(Vec<Part>);
}

pub trait ByteStream: Iterator<Item = u8> {}

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