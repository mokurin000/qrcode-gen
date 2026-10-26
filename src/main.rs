//! Application entry point.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use main::Result;

/// Desktop entry point (Windows / Linux / macOS).
#[cfg(not(target_os = "android"))]
fn main() -> Result<()> {
    use winio::prelude::*;

    use main::APP_ID;
    use main::model::MainModel;
    use main::timer::Timer;

    let init = Timer::default();

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
