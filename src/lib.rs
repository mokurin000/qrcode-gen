pub mod model;
pub mod startup;
#[cfg(windows)]
pub mod windows;

pub type Result<T> = std::result::Result<T, color_eyre::Report>;
pub const APP_ID: &str = "io.github.mokurin000.qrcode_gen";
