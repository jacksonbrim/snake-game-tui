use crate::game::SnakeGameModel;
use crate::screen::snake_screen;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io::{self, stdout};

#[derive(Debug)]
pub struct Application {
    model: SnakeGameModel,
}

impl Application {
    pub fn new() -> Self {
        Application {
            model: SnakeGameModel::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let mut should_quit = false;

        while !should_quit {
            let _ = terminal.draw(|f| self.ui(f));
            should_quit = self.model.handle_events()?;
        }
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
    fn ui(&mut self, frame: &mut Frame<'_>) {
        let _ = snake_screen(frame, &self.model);
    }
}
