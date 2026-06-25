//! Root module of the QR code generator app.

pub mod model;
pub mod timer;
#[cfg(windows)]
pub mod windows;

/// Shorthand for `Result<T, color_eyre::Report>`.
pub type Result<T> = std::result::Result<T, color_eyre::Report>;
/// Unique ID for this application.
pub const APP_ID: &str = "io.github.mokurin000.qrcode_gen";

#[cfg(target_os = "android")]
mod android;
