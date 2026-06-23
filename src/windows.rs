use std::{
    ffi::c_void,
    io::{stderr, stdin},
    os::windows::io::AsRawHandle,
};

use spdlog::{error, info};

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

const ENABLE_PROCESSED_OUTPUT: DWORD = 0b001;
const ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0b010;
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0b100;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn AttachConsole(dwProcessId: DWORD) -> BOOL;

    fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: *mut DWORD) -> BOOL;
    fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
}

/// Try attach to parent process's Console.
pub(crate) fn try_attach_console() -> WinResult<()> {
    unsafe { AttachConsole(-1_i32 as u32) }.or_error("attach console")
}

/// Try to enable VT100 support.
///
/// color-eyre would not detect this, enable VT100 to avoid garbage sequence output.
pub(crate) fn setup_virtual_terminal() {
    for handle in [stdin().as_raw_handle(), stderr().as_raw_handle()] {
        unsafe {
            let mut mode: DWORD = 0;

            _ = GetConsoleMode(handle, &mut mode).or_error("get console mode");

            // According to MSDN, you must set `0x1` for `0x4` to work.
            mode |= ENABLE_PROCESSED_OUTPUT;
            mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;

            // Workaround
            // in PowerShell, the stdout handle SetConsoleMode fails,
            // raising 'invalid param'.
            mode |= ENABLE_WRAP_AT_EOL_OUTPUT;

            info!("masked ConsoleMode: {mode:010b}");

            _ = SetConsoleMode(handle, mode).or_error("enable VT100");
        }
    }
}
