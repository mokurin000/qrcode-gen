use std::{
    ffi::c_void,
    io::{stderr, stdin},
    os::windows::io::AsRawHandle,
};

use spdlog::error;

type HANDLE = *mut c_void;
type DWORD = u32;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct BOOL(i32);

type WinResult<T> = Result<T, std::io::Error>;

impl BOOL {
    fn or_error(self, case: &str) -> WinResult<()> {
        if self.0 == 0 {
            let error = std::io::Error::last_os_error();
            error!("Failed to {case}: {error}");
            return Err(error);
        } else {
            Ok(())
        }
    }
}

const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0b100;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn AttachConsole(dwProcessId: DWORD) -> BOOL;
}

pub(crate) fn try_attach_console() -> WinResult<()> {
    unsafe { AttachConsole(-1_i32 as u32) }.or_error("attach console")
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: *mut DWORD) -> BOOL;
    fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
}

/// Try to enable VT100 support
pub(crate) fn setup_virtual_terminal() -> WinResult<()> {
    for handle in [stdin().as_raw_handle(), stderr().as_raw_handle()] {
        unsafe {
            let mut mode: DWORD = 0;

            GetConsoleMode(handle, &mut mode).or_error("get console mode")?;

            mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;

            SetConsoleMode(handle, mode).or_error("enable VT100")?;
        }
    }

    Ok(())
}
