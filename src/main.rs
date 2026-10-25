use crossterm::event::Event;
use misstep::Result;

use crate::app::App;

mod app;
mod cards;
mod ui;

fn main() -> Result<()> {
    let mut app = App::new()?;

    ratatui::run(|terminal| -> misstep::Result<()> {
        while app.running {
            terminal.draw(|frame| ui::render(&app, frame))?;
            match crossterm::event::read()? {
                Event::Key(key_event) if key_event.is_press() => app.handle_keypress(&key_event),
                _ => {}
            }
        }

        Ok(())
    })?;

    Ok(())
}
