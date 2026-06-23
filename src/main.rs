#![windows_subsystem = "windows"]

use spdlog::prelude::*;
use winio::prelude::*;

use crate::model::MainModel;
use crate::startup::Startup;

mod model;
mod startup;
#[cfg(windows)]
mod windows;

type Result<T> = std::result::Result<T, color_eyre::Report>;
const APP_ID: &str = "io.github.mokurin000.qrcode_gen";

fn main() -> Result<()> {
    let init = Startup::default();

    // We filter log levels at compile-time.
    spdlog::default_logger().set_level_filter(LevelFilter::All);

    // Try attach to console on Windows.
    //
    // By default no console window pop-up's, only if we have
    // a parent process with console attached, we output logs
    // to the terminal.
    #[cfg(windows)]
    {
        _ = windows::try_attach_console();
        windows::setup_virtual_terminal();
    }

    color_eyre::install()?;

    Ok(App::builder()
        .name(APP_ID)
        .build()?
        .block_on(MainModel::run_until_event(init))?)
}
