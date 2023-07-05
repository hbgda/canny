#[macro_export]
macro_rules! read_process {
    ($proc:literal) => {
        {
            let proc_w = windows::w!($proc);
            todo!()
        }
    };
}

#[cfg(windows)]
pub mod windows;
#[cfg(linux)]
pub mod linux;