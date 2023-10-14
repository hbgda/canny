// #[cfg(feature = "mem")]
pub mod mem;

use std::fmt::Debug;

#[cfg(windows)]
pub use windows::s;

pub mod pattern {
    use std::{ops::Deref, error::Error};

    #[derive(Debug, PartialEq, Clone)]
    pub enum Part {
        Byte(u8),
        Take,
        Skip
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Pattern(pub(crate) Vec<Part>);

    impl Pattern {
        pub fn new(pattern: &str) -> Result<Pattern, Box<dyn Error>> {
            let parts: Result<Vec<Part>, Box<dyn Error>> = pattern.to_ascii_uppercase()
                .split(" ")
                .map(|c| 
                    Ok(match c { 
                        "??" => Part::Skip,
                        "**" => Part::Take,
                        _ => Part::Byte(u8::from_str_radix(c, 16)?)
                        // _ => {
                        //     if c.len() != 2 || !c.bytes().all(|b| b.is_ascii_alphanumeric()) {
                        //         return Err(format!("Invalid pattern part: {c}"));
                        //     }
                        //     let mut bytes = c.bytes()
                        //         .map(|mut f| { if f.is_ascii_alphabetic() { f -= 7 } f - 48 })
                        //         .collect::<Vec<_>>();

                        //     bytes[0] *= 16;
                        //     Part::Byte(
                        //         bytes.iter().sum()
                        //     )
                        // }
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

pub trait ByteStream: Iterator<Item = u8> { }
impl<T: Iterator<Item = u8>> ByteStream for T { }

#[derive(Debug)]
pub struct Scanner<T: ByteStream> {
    bytes: T,
    pattern: pattern::Pattern,
    idx: usize,
    pub store: Vec<u8>
}

impl<T: ByteStream + Debug> Scanner<T> {
    pub fn scan(bytes: T, pattern: pattern::Pattern) -> Scanner<T> {
        Scanner {
            bytes,
            pattern,
            idx: 0,
            store: Vec::new()
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
        let mut store = Vec::new();
        for part in self.pattern.iter() {
            let source_byte = self.bytes.next()?;
            match *part {
                pattern::Part::Byte(byte) => {
                    if source_byte != byte {
                        self.idx += 1;
                        return self.next()
                    }
                },
                pattern::Part::Take => {
                    store.push(source_byte);
                },
                pattern::Part::Skip => {
                    continue
                },
            }
            // let pattern::Part::Byte(pattern_byte) = *part else { self.bytes.next(); continue };
            // if self.bytes.next()? != pattern_byte { self.idx += 1; return self.next(); }
        }
        self.store = store;
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
        assert_eq!(Pattern(vec![Part::Skip, Part::Byte(0x02), Part::Byte(0xA1), Part::Byte(0x9B), Part::Byte(0xFF)]), pattern.unwrap())
    }

    #[test]
    fn scan_ptr() {
        let bytes: [u8; 5] = [0x4, 0x2, 0x0, 0x6, 0x9];
        let ptr = &bytes[0] as *const u8;
        let mut scanner = Scanner::scan_ptr(ptr, 5, Pattern::new("04 ?? 00 ?? 09").unwrap());
        assert_eq!(Some(0), scanner.next())
    }

    #[test]
    fn take() {
        let bytes: [u8; 5] = [0x4, 0x2, 0x9, 0x1, 0x8];
        let mut scanner = Scanner::scan(bytes.into_iter(), Pattern::new("04 ** 09 ?? **").unwrap());
        let found = scanner.next();
        assert_eq!(Some(0), found);
        assert_eq!(scanner.store, vec![0x2, 0x8]);
    }
}