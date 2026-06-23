#![windows_subsystem = "windows"]

use spdlog::prelude::*;
use winio::prelude::*;

use qrcode_gen::model::MainModel;
use qrcode_gen::startup::Startup;
use qrcode_gen::{APP_ID, Result};

fn main() -> Result<()> {
    let init = Startup::default();

    // Try attach to console on Windows.
    //
    // By default no console window pop-up's, only if we have
    // a parent process with console attached, we output logs
    // to the terminal.
    #[cfg(windows)]
    {
        use qrcode_gen::windows::try_attach_console;

        _ = try_attach_console();
    }

    // We filter log levels at compile-time.
    spdlog::default_logger().set_level_filter(LevelFilter::All);

    color_eyre::install()?;

    Ok(App::builder()
        .name(APP_ID)
        .build()?
        .block_on(MainModel::run_until_event(init))?)
}
