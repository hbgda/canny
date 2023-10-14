#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
#[macro_export]
macro_rules! internal {
    ($proc:literal) => {
        $crate::mem::windows::ProcessReader::internal($crate::s!($proc))
    };
}
pub use internal;

#[cfg(linux)]
pub mod linux;