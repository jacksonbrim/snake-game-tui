use std::io;

mod app;
mod game;
mod screen;

use app::Application;

fn main() -> io::Result<()> {
    let mut app = Application::new();
    app.run()?;
    Ok(())
}
