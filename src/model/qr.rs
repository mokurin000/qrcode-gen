use image::{DynamicImage, Rgba};
use qrcode::render::Pixel as _;
use qrcode::types::QrError;
use qrcode::{EcLevel, QrCode, Version};
use spdlog::error;
use winio::prelude::*;

use crate::Result;
use crate::model::MainModel;
#[cfg(feature = "timing")]
use crate::timer::Timer;

impl MainModel {
    /// Update QR code, foottip and draw it on the canvas.
    pub(crate) fn update_qr(&mut self) -> Result<()> {
        let is_dark = ColorTheme::current()? == ColorTheme::Dark;

        let ec_level = self.ec_level()?;
        let version = self.version()?;
        let data = self.textbox.text()?;

        let qr = self.make_qr(ec_level, version, data);
        if let Ok(qrcode) = &qr {
            self.draw_qr(qrcode, is_dark)?;
        }
        self.qrcode = Some(qr);

        Ok(())
    }

    /// Draw the QR code onto the canvas.
    ///
    /// Cache the QR code to avoid unneeded image generation.
    fn draw_qr(&mut self, qr: &qrcode::QrCode, is_dark: bool) -> Result<()> {
        let mut dark = Rgba::default_color(qrcode::Color::Dark);
        let mut light = Rgba::default_color(qrcode::Color::Light);
        if is_dark {
            std::mem::swap(&mut dark, &mut light);
        }

        let actual_size = self.canvas.size()?;
        let max_dim = actual_size.height.min(actual_size.width) as u32;

        let mut render = qr.render::<image::Rgba<u8>>();
        let render = render
            .dark_color(dark)
            .light_color(light)
            .max_dimensions(max_dim, max_dim);

        let mut ctx = self.canvas.context()?;

        let image = {
            #[cfg(feature = "timing")]
            let _timer = Timer::with_tip("Built DrawingImage");
            let qr_image = render.build();
            let img = DynamicImage::ImageRgba8(qr_image);
            ctx.create_image(img)?
        };

        let qr_size = image.size()?;
        let left_top = (actual_size - qr_size) / 2.0;
        let rect = Rect::new(Point::origin() + left_top, qr_size);

        #[cfg(all(debug_assertions, target_os = "android"))]
        {
            let brush = SolidColorBrush::new(if is_dark {
                Color::new(255, 255, 255, 255)
            } else {
                Color::new(0, 0, 0, 255)
            });
            let pen = BrushPen::new(&brush, 1.0);
            ctx.draw_line(&pen, Point::origin(), Point::origin() + actual_size)?;
        }

        ctx.draw_image(&image, rect, None)?;

        Ok(())
    }

    fn version(&self) -> Result<Option<Version>> {
        Ok(self.version.selection()?.and_then(|ver| match ver {
            _ if ver == 0 => None,
            _ if ver <= 40 => Some(Version::Normal(ver as _)),
            _ => Some(Version::Micro(ver as i16 - 40)),
        }))
    }

    fn ec_level(&self) -> Result<EcLevel> {
        Ok(match self.eclevel.selection()? {
            None | Some(0) => EcLevel::L,
            Some(1) => EcLevel::M,
            Some(2) => EcLevel::Q,
            Some(3) => EcLevel::H,
            _ => unreachable!(),
        })
    }

    fn make_qr(
        &mut self,
        ec_level: EcLevel,
        version: Option<Version>,
        data: String,
    ) -> std::result::Result<QrCode, QrError> {
        if let Some(qr) = self.qrcode.take() {
            return qr;
        }

        #[cfg(feature = "timing")]
        let _timer = Timer::with_tip("Encoded QR code");

        let qr = if let Some(version) = version {
            qrcode::QrCode::with_version(data, version, ec_level)
        } else {
            qrcode::QrCode::with_error_correction_level(data, ec_level)
        };

        _ = self.update_foottip(&qr, version, ec_level);

        qr
    }

    fn update_foottip(
        &mut self,
        qr: &std::result::Result<QrCode, QrError>,
        version: Option<Version>,
        ec_level: EcLevel,
    ) -> Result<()> {
        match &qr {
            Ok(qr) => {
                let ec_level = qr.error_correction_level();
                match qr.version() {
                    Version::Normal(v) => {
                        self.foottip
                            .set_text(format!("Version: {v}, EcLevel: {ec_level:?}"))?;
                    }
                    // https://www.qrcode.com/en/codes/microqr.html
                    // M1 does not support any error corrction
                    // M2~M3 supports only L, M
                    // M4 supports only L, M, Q
                    Version::Micro(v) if v <= 1 => {
                        self.foottip.set_text(format!("Version: M{v}"))?;
                    }
                    Version::Micro(v) => {
                        self.foottip
                            .set_text(format!("Version: M{v}, EcLevel: {ec_level:?}"))?;
                    }
                }
            }
            Err(e) => {
                error!("Cannot generate QR: {e}");

                if let Some(Version::Micro(v)) = version {
                    match (v, ec_level) {
                        (1, EcLevel::L) => (),
                        (2 | 3, EcLevel::L | EcLevel::M) => (),
                        (4, EcLevel::L | EcLevel::M | EcLevel::Q) => (),
                        _ => {
                            self.foottip.set_text(format!(
                                "Error: EC level {ec_level:?} not supported in M{v}"
                            ))?;
                        }
                    };
                }

                self.foottip.set_text(format!("Error: {e}"))?;
            }
        }

        Ok(())
    }
}
