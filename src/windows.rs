use spdlog::error;

type DWORD = u32;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct BOOL(i32);

pub type WinResult<T> = Result<T, std::io::Error>;

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

#[link(name = "kernel32")]
unsafe extern "system" {
    fn AttachConsole(dwProcessId: DWORD) -> BOOL;
}

/// Try attach to parent process's Console.
pub fn try_attach_console() -> WinResult<()> {
    unsafe { AttachConsole(-1_i32 as u32) }.or_error("attach console")
}
