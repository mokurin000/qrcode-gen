//! Application entry point.

#![windows_subsystem = "windows"]

use main::Result;

/// Desktop entry point (Windows / Linux / macOS).
#[cfg(not(target_os = "android"))]
fn main() -> Result<()> {
    use winio::prelude::*;

    use main::APP_ID;
    use main::model::MainModel;
    use main::timer::Timer;

    let init = Timer::default();

    // Try to attach to the parent console on Windows.
    // By default no console window pops up. If the parent
    // process has a console, logs go to the terminal.
    #[cfg(windows)]
    let _ = main::windows::try_attach_console();

    App::builder()
        .name(APP_ID)
        .build()?
        .block_on(MainModel::run_until_event(init))
}

/// Android entry point is `android_main` instead.
#[cfg(target_os = "android")]
fn main() -> Result<()> {
    unreachable!("Android entry point is `android_main` in `android.rs`")
}
