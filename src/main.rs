#![windows_subsystem = "windows"]

use main::Result;

#[cfg(not(target_os = "android"))]
fn main() -> Result<()> {
    use spdlog::prelude::*;
    use winio::prelude::*;

    use main::APP_ID;
    use main::model::MainModel;
    use main::timer::Timer;

    let init = Timer::default();

    // Try attach to console on Windows.
    //
    // By default no console window pop-up's, only if we have
    // a parent process with console attached, we output logs
    // to the terminal.
    #[cfg(windows)]
    let _ = main::windows::try_attach_console();

    // We filter log levels at compile-time.
    spdlog::default_logger().set_level_filter(LevelFilter::All);
    // color-eyre would not enable VT100 support on Windows
    color_eyre::install()?;

    Ok(App::builder()
        .name(APP_ID)
        .build()?
        .block_on(MainModel::run_until_event(init))?)
}

#[cfg(target_os = "android")]
fn main() -> Result<()> {
    unreachable!("Android entry point is `android_main` in `android.rs`")
}
