//! Main GUI component for the QR code generator.

use qrcode::QrCode;
use qrcode::types::QrError;
use winio::prelude::*;

use crate::Result;
use crate::timer::Timer;

/// Root component of the application UI.
pub struct MainModel {
    /// The main application window.
    window: Child<Window>,

    /// Drop-down to select error correction level.
    eclevel: Child<ComboBox>,
    /// Drop-down to select QR code version.
    version: Child<ComboBox>,
    /// Input field for the text to encode.
    textbox: Child<TextBox>,
    /// Area where the QR code image is drawn.
    canvas: Child<Canvas>,
    /// Status text shown below the canvas.
    foottip: Child<Label>,

    /// Cached QR code result (None = needs regeneration).
    qrcode: Option<std::result::Result<QrCode, QrError>>,
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
    type Init<'a> = Timer;
    type Message = MainMessage;

    async fn init(_init: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        // create & initialize the window
        init! {
            window: Window = (()) => {
                text: "QRCode Generator",
                size: Size::new(800.0, 600.0),

                #[cfg(all(windows, feature = "winui"))]
                backdrop: Backdrop::Mica,
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

        window.show()?;

        Ok(Self {
            window,
            textbox,
            eclevel,
            version,
            canvas,
            foottip,
            qrcode: None,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        // listen to events
        start! {
            sender, default: MainMessage::Noop,
            self.window => {
                WindowEvent::Resize => MainMessage::Resize,
                WindowEvent::Close => MainMessage::Close,
                WindowEvent::ThemeChanged => MainMessage::ContentChanged,
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
                self.qrcode.take();
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

        self.update_qr()?;

        Ok(())
    }

    fn render_children(&mut self) -> Result<()> {
        Ok(self.window.render()?)
    }
}

// QR code generation and drawing logic.
mod qr;
