use image::{DynamicImage, Rgba};
use qrcode::render::Pixel as _;
use winio::prelude::*;

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
}
