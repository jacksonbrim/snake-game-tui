//use crate::models::{
//apigw::ApiGwViewModel, cfn::StacksViewModel, identity::CognitoIdentityViewModel,
//lambda_service::LambdaViewModel, resources::ResourcesViewModel, s3_service::S3ViewModel,
//};
use crate::game::SnakeGameViewModel;
use crate::screen::snake_screen;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io::{self, stdout};

#[derive(Debug)]
pub struct Application {
    view_model: SnakeGameViewModel,
}

impl Application {
    pub fn new() -> Self {
        Application {
            view_model: SnakeGameViewModel::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let mut should_quit = false;

        while !should_quit {
            let _ = terminal.draw(|f| self.ui(f));
            should_quit = self.handle_events()?;
        }
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<bool> {
        self.view_model.handle_events()
    }
    fn ui(&mut self, frame: &mut Frame<'_>) {
        let _ = snake_screen(frame, &self.view_model);
    }
}
