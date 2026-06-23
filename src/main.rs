#![windows_subsystem = "windows"]

use spdlog::prelude::*;
use winio::prelude::*;

use crate::startup::Startup;

mod startup;
#[cfg(windows)]
mod windows;

type Result<T> = std::result::Result<T, color_eyre::Report>;
const APP_ID: &str = "io.github.mokurin000.qrcode_gen";

fn main() -> Result<()> {
    let init = Startup::default();

    // We filter log levels at compile-time.
    spdlog::default_logger().set_level_filter(LevelFilter::All);

    // Try attach to console on Windows.
    //
    // By default no console window pop-up's, only if we have
    // a parent process with console attached, we output logs
    // to the terminal.
    #[cfg(windows)]
    {
        _ = windows::try_attach_console();
        windows::setup_virtual_terminal();
    }

    color_eyre::install()?;

    Ok(App::builder()
        .name(APP_ID)
        .build()?
        .block_on(MainModel::run_until_event(init))?)
}

struct MainModel {
    window: Child<Window>,
}

enum MainMessage {
    Noop,
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
                text: "Example",
                size: Size::new(800.0, 600.0),
            }
        }

        window.show()?;

        Ok(Self { window })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        // listen to events
        start! {
            sender, default: MainMessage::Noop,
            self.window => {
                WindowEvent::Close => MainMessage::Close,
            }
        }
    }

    async fn update_children(&mut self) -> Result<bool> {
        // update the window
        update_children!(self.window)
    }

    async fn update(
        &mut self,
        message: Self::Message,
        sender: &ComponentSender<Self>,
    ) -> Result<bool> {
        // deal with custom messages
        match message {
            MainMessage::Noop => Ok(false),
            MainMessage::Close => {
                // the root component output stops the application
                sender.output(());
                // need not to call `render`
                Ok(false)
            }
        }
    }

    fn render(&mut self, _sender: &ComponentSender<Self>) -> Result<()> {
        // let csize = self.window.client_size()?;
        // adjust layout and draw widgets here
        Ok(())
    }

    fn render_children(&mut self) -> Result<()> {
        Ok(self.window.render()?)
    }
}
