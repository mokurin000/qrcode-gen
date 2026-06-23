use winio::prelude::*;

use crate::Result;
use crate::startup::Startup;

pub struct MainModel {
    window: Child<Window>,
    canvas: Child<Canvas>,
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
                text: "Example",
                size: Size::new(800.0, 600.0),
            },
            canvas: Canvas = (&window),
        }

        #[cfg(windows)]
        window.set_backdrop(Backdrop::Mica)?;

        window.show()?;

        Ok(Self { window, canvas })
    }

    async fn start(&mut self, sender: &ComponentSender<Self>) -> ! {
        // listen to events
        start! {
            sender, default: MainMessage::Noop,
            self.window => {
                WindowEvent::Resize => MainMessage::ReDraw,
                WindowEvent::Close => MainMessage::Close,
            }
        }
    }

    async fn update_children(&mut self) -> Result<bool> {
        // update the window
        update_children!(self.window, self.canvas,)
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
        self.canvas.set_size(csize)?;

        let mut ctx = self.canvas.context()?;
        let is_dark = ColorTheme::current()? == ColorTheme::Dark;
        let brush = SolidColorBrush::new(if is_dark {
            Color::new(255, 255, 255, 255)
        } else {
            Color::new(0, 0, 0, 255)
        });
        let pen = BrushPen::new(&brush, 1.0);

        ctx.draw_line(&pen, Point::zero(), Point::new(csize.width, csize.height))?;
        ctx.draw_line(
            &pen,
            Point::new(0.0, csize.height),
            Point::new(csize.width, 0.0),
        )?;

        Ok(())
    }

    fn render_children(&mut self) -> Result<()> {
        Ok(self.window.render()?)
    }
}
