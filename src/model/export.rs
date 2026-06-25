use compio::BufResult;
use compio::io::AsyncWriteAtExt;
use compio::runtime::{spawn, spawn_blocking};
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::{ExtendedColorType, ImageEncoder as _, Rgba};
use spdlog::error;
use winio::prelude::*;

use crate::timer::Timer;

use crate::Result;
use crate::model::MainModel;

impl MainModel {
    /// Export the QR code to *.png
    pub(crate) async fn export_png(&self) -> Result<()> {
        let Some(png_file) = FileBox::new()
            .add_filter(("PNG Images", "*.png"))
            .filename("qrcode.png")
            .title(self.format_ftl("export-png-tooltip", None))
            .save(&self.window)
            .await?
        else {
            // user cancellation
            return Ok(());
        };

        let Some(Ok(qr)) = &self.qrcode else {
            return Ok(());
        };
        let qr = qr.clone();

        spawn(async move {
            let img = spawn_blocking(move || {
                let img = qr.render::<Rgba<u8>>().max_dimensions(1024, 1024).build();

                let _timer = Timer::with_tip("encoded QR code to png");
                let mut png_buf = Vec::<u8>::new();
                let png = PngEncoder::new_with_quality(
                    &mut png_buf,
                    CompressionType::Best,
                    FilterType::NoFilter,
                );
                png.write_image(
                    img.as_raw(),
                    img.width(),
                    img.height(),
                    ExtendedColorType::Rgba8,
                )
                .inspect_err(|e| {
                    error!("Failed to encode png: {e}");
                })?;

                Result::Ok(png_buf)
            })
            .await
            .expect("thread join error")?;

            let mut uni_file = UriFile::create(png_file).await.inspect_err(|e| {
                error!("Failed to create the png file: {e}");
            })?;

            let BufResult(result, _) = uni_file.write_all_at(img, 0).await;

            if let Err(e) = result {
                error!("Failed to write the png file: {e}");
            }

            Result::Ok(())
        })
        .detach();

        Ok(())
    }
}
