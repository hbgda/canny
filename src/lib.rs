// #[cfg(feature = "mem")]
pub mod mem;

use std::fmt::Debug;

pub mod pattern {
    use std::ops::Deref;

    #[derive(Debug, PartialEq)]
    pub enum Part {
        Byte(u8),
        Skip
    }

    #[derive(Debug, PartialEq)]
    pub struct Pattern(pub(crate) Vec<Part>);

    impl Pattern {
        pub fn new(pattern: &str) -> Result<Pattern, String> {
            let parts: Result<Vec<Part>, String> = pattern.to_ascii_uppercase()
                .split(" ")
                .map(|c| 
                    Ok(match c { 
                        "??" => Part::Skip, 
                        _ => {
                            if c.len() != 2 || !c.bytes().all(|b| b.is_ascii_alphanumeric()) {
                                return Err(format!("Invalid pattern part: {c}"));
                            }
                            let mut bytes = c.bytes()
                                .map(|mut f| { if f.is_ascii_alphabetic() { f -= 7 } f - 48 })
                                .collect::<Vec<_>>();

                            bytes[0] *= 16;
                            Part::Byte(
                                bytes.iter().sum()
                            )
                        }
                    })
                )
                .collect();

            Ok(Pattern(parts?))
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
    pub fn scan(bytes: T, pattern: pattern::Pattern) -> Scanner<T> {
        Scanner {
            bytes_len: bytes.clone().count(),
            bytes,
            pattern,
            idx: 0
        }
    }
}

impl Scanner<ScanPtr> {
    /// Convenience function for `Scanner::scan(ScanPtr { ptr, offset, len }, / ... /)`
    pub fn scan_ptr(ptr: *const u8, len: usize, pattern: pattern::Pattern) -> Scanner<ScanPtr> {
        Scanner::scan(ScanPtr { ptr, offset: 0, len }, pattern)
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
            if self.bytes.next()? != pattern_byte { self.idx += 1; return self.next(); }
        }

        Some(self.idx)
    }
}

#[derive(Debug, Clone)]
pub struct ScanPtr {
    ptr: *const u8,
    offset: usize,
    len: usize
}

impl Iterator for ScanPtr {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.len {
            return None;
        }
        unsafe {
            let val = Some(self.ptr.add(self.offset).read());
            self.offset += 1;
            val
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::{Scanner, pattern::{Pattern, Part}};

    #[test]
    fn scan() {
        let bytes = vec![0x3, 0x12, 0x58, 0xFF, 0x0, 0x1, 0x2, 0x3];
        let mut scanner = Scanner::scan(bytes.into_iter(), Pattern::new("FF ?? 01").unwrap());
        let found = scanner.next();
        assert_eq!(Some(3), found);
    }

    #[test]
    fn pattern_new() {
        let pattern = Pattern::new("?? 02 A1 9B FF");
        assert_eq!(Ok(Pattern(vec![Part::Skip, Part::Byte(0x02), Part::Byte(0xA1), Part::Byte(0x9B), Part::Byte(0xFF)])), pattern)
    }

    #[test]
    fn scan_ptr() {
        let bytes: [u8; 5] = [0x4, 0x2, 0x0, 0x6, 0x9];
        let ptr = &bytes[0] as *const u8;
        let mut scanner = Scanner::scan_ptr(ptr, 5, Pattern::new("04 ?? 00 ?? 09").unwrap());
        assert_eq!(Some(0), scanner.next())
    }

    #[test]
    fn mem_windows() {
        unsafe {
            let process_mem = crate::read_process!("Steam");
        }
    }
}