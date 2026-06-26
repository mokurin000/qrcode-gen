//! Main GUI component for the QR code generator.

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use qrcode::QrCode;
use qrcode::types::QrError;
use spdlog::{error, info};
use winio::prelude::*;

use crate::Result;
use crate::i18n::format_ftl;
use crate::timer::Timer;

/// Margin size of widgets
const MARGIN: f64 = 5.0;
/// Margin size of the canvas
const MARGIN_CANVAS: f64 = 10.0;

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
    status: Child<Label>,

    /// Button to export QR code as PNG.
    export_png: Child<Button>,
    /// Button to export QR code as SVG.
    export_svg: Child<Button>,

    /// Cached QR code result (None = needs regeneration).
    qrcode: Option<std::result::Result<QrCode, QrError>>,

    /// Fluent i18n bundle for the resolved system locale.
    bundle: FluentBundle<FluentResource>,

    #[cfg(target_os = "android")]
    radius: f64,
}

pub enum MainMessage {
    /// Nothing to do
    Noop,
    /// Main window has been resized
    Resize,
    /// Theme changed
    ThemeChanged,
    /// QRCode must be updated
    ContentChanged,
    /// Close main window
    Close,
    /// Export QR code as PNG
    ExportQRcodePNG,
    /// Export QR code as SVG
    ExportQRcodeSVG,
}

impl Component for MainModel {
    type Error = color_eyre::Report;
    type Event = ();
    type Init<'a> = Timer;
    type Message = MainMessage;

    async fn init(_init: Self::Init<'_>, _sender: &ComponentSender<Self>) -> Result<Self> {
        // Resolve system locale and load the Fluent bundle before creating widgets.
        let sys_locale = sys_locale::get_locale().unwrap_or_else(|| "en-US".into());
        let locale = crate::i18n::resolve_locale(&sys_locale);
        info!("decided locale: {locale}");
        let bundle = crate::i18n::load_bundle(&locale.to_string())?;

        // Format all localized strings from the bundle.
        let title = format_ftl(&bundle, "window-title", None);
        let ec_tooltip = format_ftl(&bundle, "ec-level-tooltip", None);
        let version_tooltip = format_ftl(&bundle, "version-tooltip", None);
        let textbox_tooltip = format_ftl(&bundle, "textbox-tooltip", None);
        let status_tooltip = format_ftl(&bundle, "status-tooltip", None);
        let version_auto = format_ftl(&bundle, "version-auto", None);

        let mut png_args = FluentArgs::new();
        png_args.set("format", "png");
        let export_png_text = format_ftl(&bundle, "export-file", Some(&png_args));
        let export_png_tooltip = format_ftl(&bundle, "export-tooltip", Some(&png_args));
        let mut svg_args = FluentArgs::new();
        svg_args.set("format", "svg");
        let export_svg_text = format_ftl(&bundle, "export-file", Some(&svg_args));
        let export_svg_tooltip = format_ftl(&bundle, "export-tooltip", Some(&svg_args));

        // create & initialize the window with localized strings
        init! {
            window: Window = (()) => {
                text: title,
                size: Size::new(800.0, 600.0),

                #[cfg(all(windows, feature = "winui"))]
                backdrop: Backdrop::Mica,
            },
            canvas: Canvas = (&window),
            eclevel: ComboBox = (&window) => {
                items: ["L (7%)", "M (15%)", "Q (25%)", "H (30%)"],
                tooltip: ec_tooltip,
            },
            version: ComboBox = (&window) => {
                items: [version_auto].into_iter()
                    .chain((1..=40).map(|n| n.to_string()))
                    .chain(
                        [
                            "M1".into(),
                            "M2".into(),
                            "M3".into(),
                            "M4".into(),
                        ]
                    ),
                tooltip: version_tooltip,
            },
            textbox: TextBox = (&window) => {
                tooltip: textbox_tooltip,
            },
            status: Label = (&window) => {
                halign: HAlign::Center,
                tooltip: status_tooltip,
            },
            export_png: Button = (&window) => {
                text: export_png_text,
                tooltip: export_png_tooltip,
            },
            export_svg: Button = (&window) => {
                text: export_svg_text,
                tooltip: export_svg_tooltip,
            }
        }

        #[cfg(target_os = "android")]
        let radius = {
            use jni::{jni_sig, jni_str};

            let obj = window.as_window().to_android();
            jni::vm::JavaVM::singleton()?.attach_current_thread(move |env| {
                let result = env.call_method(
                    obj,
                    jni_str!("getBottomLeftCornerRadius"),
                    jni_sig!(() -> int),
                    &[],
                )?;
                Result::Ok(result.into_int()?)
            })? as f64
        };

        window.show()?;

        Ok(Self {
            window,
            textbox,
            eclevel,
            version,
            canvas,
            status,
            bundle,
            export_png,
            export_svg,

            qrcode: None,
            #[cfg(target_os = "android")]
            radius,
        })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        // listen to events
        start! {
            sender, default: MainMessage::Noop,
            self.window => {
                WindowEvent::Resize => MainMessage::Resize,
                WindowEvent::Close => MainMessage::Close,
                WindowEvent::ThemeChanged => MainMessage::ThemeChanged,
            },
            self.textbox => {
                TextBoxEvent::Change => MainMessage::ContentChanged,
            },
            self.eclevel => {
                ComboBoxEvent::Select => MainMessage::ContentChanged,
            },
            self.version => {
                ComboBoxEvent::Select => MainMessage::ContentChanged,
            },
            self.export_png => {
                ButtonEvent::Click => MainMessage::ExportQRcodePNG,
            },
            self.export_svg => {
                ButtonEvent::Click => MainMessage::ExportQRcodeSVG,
            }
        }
    }

    async fn update_children(&mut self) -> Result<bool> {
        // update the window
        update_children!(
            self.window,
            self.textbox,
            self.canvas,
            self.eclevel,
            self.version,
            self.export_png,
            self.export_svg,
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
            MainMessage::ThemeChanged => Ok(true),
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
            MainMessage::ExportQRcodePNG => {
                if let Err(e) = self.export_png().await {
                    error!("Failed to open file dialog: {e}");
                }
                Ok(false)
            }
            MainMessage::ExportQRcodeSVG => {
                if let Err(e) = self.export_svg().await {
                    error!("Failed to open file dialog: {e}");
                }
                Ok(false)
            }
        }
    }

    fn render(&mut self, _sender: &ComponentSender<Self>) -> Result<()> {
        #[allow(unused_mut)]
        let mut csize = self.window.client_size()?;
        #[cfg(target_os = "android")]
        {
            csize.height -= self.radius;
        }

        let mut control = layout! {
            StackPanel::new(Orient::Horizontal),
            self.version => {
                grow: true,
                margin: Margin::new_all_same(MARGIN),
            },
            self.eclevel => {
                grow: true,
                margin: Margin::new(MARGIN, MARGIN, MARGIN, 0.0),
            },
        };

        let mut export = layout! {
            StackPanel::new(Orient::Horizontal),
            self.export_png => {
                grow: true,
                margin: Margin::new_all_same(MARGIN),
            },
            self.export_svg => {
                grow: true,
                margin: Margin::new(MARGIN, MARGIN, MARGIN, 0.0),
            },
        };

        let mut panel = layout! {
            StackPanel::new(Orient::Vertical),
            self.textbox => {
                margin: Margin::new_all_same(MARGIN),
            },
            control,
            self.canvas => {
                grow: true,
                margin: Margin::new_all_same(MARGIN_CANVAS),
            },
            self.status,
            export,
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

mod export;
