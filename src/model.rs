use winio::prelude::*;

use crate::Result;
use crate::startup::Startup;

pub struct MainModel {
    window: Child<Window>,
}

pub enum MainMessage {
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
