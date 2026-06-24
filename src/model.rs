use std::mem;

use image::{DynamicImage, Rgba};
use qrcode::render::Pixel;
use qrcode::{EcLevel, Version};
use spdlog::error;
use winio::prelude::*;

use crate::Result;
use crate::startup::Startup;

pub struct MainModel {
    window: Child<Window>,
    textbox: Child<TextBox>,
    eclevel: Child<ComboBox>,
    canvas: Child<Canvas>,
    foottip: Child<Label>,
}

pub enum MainMessage {
    Noop,
    ReDraw,
    Close,
}

impl Component for MainModel {
    type Error = color_eyre::Report;
    type Event = ();
    type Init<'a> = Startup;
    type Message = MainMessage;

    async fn init(_init: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        // create & initialize the window
        init! {
            window: Window = (()) => {
                text: "QRCode Generator",
                size: Size::new(800.0, 600.0),
            },
            canvas: Canvas = (&window),
            eclevel: ComboBox = (&window) => {
                items: ["L 7%", "M 15%", "Q 25%", "H 30%"],
                tooltip: "Error correction level."
            },
            textbox: TextBox = (&window) => {
                tooltip: "Text to generate QRCode for",
            },
            foottip: Label = (&window) => {
                halign: HAlign::Center,
                tooltip: "Status of the QRCode generation",
            },
        }

        #[cfg(windows)]
        window.set_backdrop(Backdrop::Mica)?;

        window.show()?;

        Ok(Self {
            window,
            textbox,
            eclevel,
            canvas,
            foottip,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        // listen to events
        start! {
            sender, default: MainMessage::Noop,
            self.window => {
                WindowEvent::Resize => MainMessage::ReDraw,
                WindowEvent::Close => MainMessage::Close,
            },
            self.textbox => {
                TextBoxEvent::Change => MainMessage::ReDraw,
            },
            self.eclevel => {
                ComboBoxEvent::Select => MainMessage::ReDraw,
            }
        }
    }

    async fn update_children(&mut self) -> Result<bool> {
        // update the window
        update_children!(
            self.window,
            self.textbox,
            self.canvas,
            self.foottip,
            self.eclevel,
        )
    }

    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool> {
        // deal with custom messages
        match message {
            MainMessage::Noop => Ok(false),
            MainMessage::ReDraw => Ok(true),
            MainMessage::Close => {
                // the root component output stops the application
                sender.output(());
                // need not to call `render`
                Ok(false)
            }
        }
    }

    fn render(&mut self, _sender: &ComponentSender<Self>) -> Result<()> {
        let csize = self.window.client_size()?;

        {
            let mut control = layout! {
                StackPanel::new(Orient::Horizontal),
                self.eclevel => {
                    halign: HAlign::Center,
                    grow: true,
                    margin: Margin::new_all_same(2.0),
                },
            };
            let mut panel = layout! {
                StackPanel::new(Orient::Vertical),
                control,
                self.textbox,
                self.canvas => { grow: true },
                self.foottip,
            };
            panel.set_size(csize)?;
        }

        let is_dark = ColorTheme::current()? == ColorTheme::Dark;

        let ec_level = match self.eclevel.selection()? {
            None | Some(0) => EcLevel::L,
            Some(1) => EcLevel::M,
            Some(2) => EcLevel::Q,
            Some(3) => EcLevel::H,
            _ => unreachable!(),
        };

        let qr = qrcode::QrCode::with_error_correction_level(self.textbox.text()?, ec_level);
        match qr {
            Err(e) => {
                error!("Cannot generate QR: {e}");

                self.foottip.set_text(format!("Error: {e}"))?;
            }
            Ok(qr) => {
                let mut dark = Rgba::default_color(qrcode::Color::Dark);
                let mut light = Rgba::default_color(qrcode::Color::Light);
                if is_dark {
                    mem::swap(&mut dark, &mut light);
                }

                let actual_size = self.canvas.size()?;
                let max_dim = actual_size.height.min(actual_size.width) as u32;
                let qr_image = qr
                    .render::<image::Rgba<u8>>()
                    .dark_color(dark)
                    .light_color(light)
                    .max_dimensions(max_dim, max_dim)
                    .build();
                let mut ctx = self.canvas.context()?;
                let qr_size = Size::new(qr_image.width() as _, qr_image.height() as _);
                let img = DynamicImage::ImageRgba8(qr_image);
                let image = ctx.create_image(img)?;

                let left_top = (actual_size - qr_size) / 2.0;
                let rect = Rect::new(Point::new(left_top.width, left_top.height), qr_size);

                ctx.draw_image(&image, rect, Some(Rect::new(Point::origin(), qr_size)))?;

                match qr.version() {
                    Version::Normal(v) => {
                        self.foottip
                            .set_text(format!("Version: {v}, EcLevel: {ec_level:?}"))?;
                    }
                    Version::Micro(v) if v <= 2 => {
                        self.foottip.set_text(format!("Version: M{v}"))?;
                    }
                    Version::Micro(v) => {
                        self.foottip
                            .set_text(format!("Version: M{v}, EcLevel: {ec_level:?}"))?;
                    }
                }
            }
        }

        Ok(())
    }

    fn render_children(&mut self) -> Result<()> {
        Ok(self.window.render()?)
    }
}
