pub mod model;
mod qr;
pub mod startup;
#[cfg(windows)]
pub mod windows;

pub type Result<T> = std::result::Result<T, color_eyre::Report>;
pub const APP_ID: &str = "io.github.mokurin000.qrcode_gen";

#[cfg(target_os = "android")]
mod android;
