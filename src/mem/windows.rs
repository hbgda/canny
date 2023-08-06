use std::{error::Error, ffi::c_void};

use windows::Win32::{System::{LibraryLoader::{GetModuleHandleW, GetModuleHandleA}, ProcessStatus::{GetModuleInformation, MODULEINFO}, Threading::GetCurrentProcess, Memory::{MEMORY_BASIC_INFORMATION, VirtualQuery, MEM_COMMIT, PAGE_READWRITE}}, Foundation::HMODULE};

#[derive(Debug)]
pub struct ProcessReader {
    mem_base: usize,
    mem_size: usize,
    offset: usize
}

impl ProcessReader {
    /// Read memory from internal process
    pub unsafe fn internal(process: windows::core::PCSTR) -> Result<ProcessReader, Box<dyn Error>> {
        let base = GetModuleHandleA(process)?;
        let mut module_info = MODULEINFO::default();
        GetModuleInformation(GetCurrentProcess(), base, &mut module_info, std::mem::size_of::<MODULEINFO>() as u32);
        Ok(ProcessReader {
            mem_base: base.0 as usize,
            mem_size: module_info.SizeOfImage as usize,
            offset: 0
        })
    }

    /// Read memory from external process
    pub unsafe fn external(process: windows::core::PCSTR) -> ProcessReader {
        todo!()
    }
}

impl Iterator for ProcessReader {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let mut mbi = MEMORY_BASIC_INFORMATION::default();
        unsafe {
            VirtualQuery(Some((self.mem_base + self.offset) as *const c_void), &mut mbi, std::mem::size_of::<MEMORY_BASIC_INFORMATION>());
        }
        if (mbi.State & MEM_COMMIT).0 == 0 && (mbi.Protect & PAGE_READWRITE).0 == 0 {
            return None;
        } 
        self.offset += 1;
        unsafe {
            Some(std::ptr::read((self.mem_base + self.offset) as *const u8))
        }
    }
}