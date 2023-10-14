use std::{error::Error, ffi::c_void};

use windows::Win32::{System::{LibraryLoader::{GetModuleHandleW, GetModuleHandleA}, ProcessStatus::{GetModuleInformation, MODULEINFO}, Threading::GetCurrentProcess, Memory::{MEMORY_BASIC_INFORMATION, VirtualQuery, MEM_COMMIT, PAGE_READWRITE}}, Foundation::HMODULE};

use crate::pattern::{self, Part};

pub struct ProcessScanner {
    process: ProcessInfo,
    pattern: pattern::Pattern,
    pub store: Vec<u8>
}

impl ProcessScanner {
    pub fn scan(process: ProcessInfo, pattern: pattern::Pattern) -> ProcessScanner {
        ProcessScanner { process, pattern, store: Vec::new() }
    }
}

impl Iterator for ProcessScanner {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut offset: usize = 0;
        'mem_loop: while offset < self.process.mem_size {
            let mut store = Vec::new();
            let mut pattern_offset = 0usize;
            unsafe {
                for part in self.pattern.iter() {
                    let source_byte = std::ptr::read((self.process.mem_base + offset + pattern_offset) as *const u8);
                    match *part {
                        Part::Byte(byte) => {
                            if source_byte != byte {
                                offset += 1;
                                continue 'mem_loop;
                            }
                        },
                        Part::Take => {
                            store.push(source_byte);
                        },
                        Part::Skip => {},
                    }
                    pattern_offset += 1;
                    // let Part::Byte(byte) = *part else { pattern_offset += 1; continue; };
                    // if std::ptr::read((self.process.mem_base + offset + pattern_offset) as *const u8) != byte { offset += 1; continue 'mem_loop; };
                    // pattern_offset += 1;
                    // println!("{offset:#X}/{:#X} Found part: {part:#X?}", self.process.mem_size);
                    // println!("Next byte: {:#X}", std::ptr::read((self.process.mem_base + offset + pattern_offset + 1) as *const u8));
                }
            }
            self.store = store;
            return Some(self.process.mem_base + offset);
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessInfo {
    mem_base: usize,
    mem_size: usize
}

impl ProcessInfo {
    /// Read memory from internal process
    pub unsafe fn internal(process: windows::core::PCSTR) -> Result<ProcessInfo, Box<dyn Error>> {
        let base = GetModuleHandleA(process)?;
        let mut module_info = MODULEINFO::default();
        GetModuleInformation(GetCurrentProcess(), base, &mut module_info, std::mem::size_of::<MODULEINFO>() as u32);
        Ok(ProcessInfo {
            mem_base: module_info.lpBaseOfDll as usize,
            mem_size: module_info.SizeOfImage as usize
        })
    }

    /// Read memory from external process
    pub unsafe fn external(process: windows::core::PCSTR) -> ProcessInfo {
        todo!()
    }
}