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
    version: Child<ComboBox>,
    canvas: Child<Canvas>,
    foottip: Child<Label>,

    drawing_img: Option<((u32, u32), DrawingImage)>,
}

pub enum MainMessage {
    /// Nothing to do
    Noop,
    /// Main window has been resized
    Resize,
    /// QRCode must be updated
    ContentChanged,
    /// Close main window
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
                items: ["L (7%)", "M (15%)", "Q (25%)", "H (30%)"],
                tooltip: "Error correction level."
            },
            version: ComboBox = (&window) => {
                items: ["Auto".to_string()].into_iter()
                    .chain((1..=40).map(|n| n.to_string()))
                    .chain(
                        [
                            "M1".into(),
                            "M2".into(),
                            "M3".into(),
                            "M4".into(),
                        ]
                    ),
                tooltip: "QRCode spec version."
            },
            textbox: TextBox = (&window) => {
                tooltip: "Text to generate QRCode for.",
            },
            foottip: Label = (&window) => {
                halign: HAlign::Center,
                tooltip: "Status of the QRCode generation.",
            },
        }

        #[cfg(all(windows, feature = "winui"))]
        window.set_backdrop(Backdrop::Mica)?;

        window.show()?;

        Ok(Self {
            window,
            textbox,
            eclevel,
            version,
            canvas,
            foottip,
            drawing_img: None,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        // listen to events
        start! {
            sender, default: MainMessage::Noop,
            self.window => {
                WindowEvent::Resize  => MainMessage::Resize,
                WindowEvent::Close => MainMessage::Close,
            },
            self.textbox => {
                TextBoxEvent::Change => MainMessage::ContentChanged,
            },
            self.eclevel => {
                ComboBoxEvent::Select => MainMessage::ContentChanged,
            },
            self.version => {
                ComboBoxEvent::Select => MainMessage::ContentChanged,
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
            self.version,
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
            MainMessage::Resize => Ok(true),
            MainMessage::ContentChanged => {
                self.drawing_img.take();
                Ok(true)
            }
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
                self.version => {
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

        let version = self.version.selection()?.and_then(|ver| match ver {
            _ if ver == 0 => None,
            _ if ver <= 40 => Some(Version::Normal(ver as _)),
            _ => Some(Version::Micro(ver as i16 - 40)),
        });
        let qr = if let Some(version) = version {
            qrcode::QrCode::with_version(self.textbox.text()?, version, ec_level)
        } else {
            qrcode::QrCode::with_error_correction_level(self.textbox.text()?, ec_level)
        };

        match qr {
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
                            return Ok(());
                        }
                    };
                }

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
        }

        Ok(())
    }

    fn render_children(&mut self) -> Result<()> {
        Ok(self.window.render()?)
    }
}
