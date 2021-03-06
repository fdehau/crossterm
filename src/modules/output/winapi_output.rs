use super::IStdout;
use kernel::windows_kernel::{writing, Handle};

use std::io;

/// This struct is a wrapper for WinApi output.
pub struct WinApiOutput;

impl WinApiOutput {
    pub fn new() -> WinApiOutput {
        WinApiOutput
    }
}

impl IStdout for WinApiOutput {
    fn write_str(&self, string: &str) -> io::Result<usize> {
        self.write(string.as_bytes())
    }

    fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let handle = Handle::current_out_handle()?;
        writing::write_char_buffer(&handle, buf)
    }

    fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

unsafe impl Send for WinApiOutput {}

unsafe impl Sync for WinApiOutput {}
