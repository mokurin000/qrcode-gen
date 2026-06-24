use image::{DynamicImage, Rgba};
use qrcode::render::Pixel as _;
use qrcode::{EcLevel, QrCode, Version};
use winio::prelude::*;

use crate::Result;
use crate::model::MainModel;

impl MainModel {
    /// Draw the QR code onto the canvas.
    pub(crate) fn draw_qr(&mut self, qr: &qrcode::QrCode, is_dark: bool) -> Result<()> {
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

        let qr_realsize = render.real_size();

        let mut ctx = self.canvas.context()?;

        let image = if let Some((realsize, i)) = self.drawing_img.take()
            && realsize == qr_realsize
        {
            i
        } else {
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

        ctx.draw_image(&image, rect, Some(Rect::new(Point::origin(), qr_size)))?;
        self.drawing_img = Some((qr_realsize, image));
        Ok(())
    }

    pub(crate) fn version(&self) -> Result<Option<Version>> {
        Ok(self.version.selection()?.and_then(|ver| match ver {
            _ if ver == 0 => None,
            _ if ver <= 40 => Some(Version::Normal(ver as _)),
            _ => Some(Version::Micro(ver as i16 - 40)),
        }))
    }

    pub(crate) fn ec_level(&self) -> Result<EcLevel> {
        Ok(match self.eclevel.selection()? {
            None | Some(0) => EcLevel::L,
            Some(1) => EcLevel::M,
            Some(2) => EcLevel::Q,
            Some(3) => EcLevel::H,
            _ => unreachable!(),
        })
    }

    pub(crate) fn make_qr(&mut self) -> Result<QrCode> {
        let ec_level = self.ec_level()?;
        let version = self.version()?;

        let qr = if let Some(version) = version {
            qrcode::QrCode::with_version(self.textbox.text()?, version, ec_level)
        } else {
            qrcode::QrCode::with_error_correction_level(self.textbox.text()?, ec_level)
        };

        Ok(qr?)
    }
}
