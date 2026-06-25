//! Application entry point.

#![windows_subsystem = "windows"]

use main::Result;

/// Desktop entry point (Windows / Linux / macOS).
#[cfg(not(target_os = "android"))]
fn main() -> Result<()> {
    use spdlog::prelude::*;
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

    // Enable all log levels (filtered at compile time).
    spdlog::default_logger().set_level_filter(LevelFilter::All);
    // color-eyre does not enable VT100 on Windows on its own.
    // Currently, spdlog-rs handled this.
    color_eyre::install()?;

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
