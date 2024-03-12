use std::marker::PhantomData;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::disable_raw_mode;
use ratatui::{Frame, Terminal};
use ratatui::backend::Backend;

use app::app_states::AppState;

use crate::app;
use crate::models::tablature::Tablature;

#[derive(Debug, Clone)]
pub struct App<'a> {
    pub tab: Option<Tablature>,
    pub should_quit: bool,
    pub state: AppState,
    phantom: PhantomData<&'a ()>,
    // Add more vars after
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            tab: Option::from(Tablature {
                name: String::new(),
                strings: Vec::new(),
            }),
            should_quit: false,
            state: AppState::Normal,
            phantom: Default::default(),
        }
    }

    pub async fn run(&mut self, mut terminal: Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.clear()?;

        while !self.should_quit {
            self.draw(&mut terminal)?;
            self.handle_events().await;
        }

        Ok(())
    }

    pub fn chain_hook(&mut self) -> &mut Self {
        let original_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic| {
            disable_raw_mode().unwrap();
            original_hook(panic);
        }));
        self
    }

    pub fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.draw(|frame| match self.state {
            AppState::Normal => self.ui(frame),
            AppState::Main => self.ui(frame),
            AppState::NewProject => self.ui_newproject(frame),
            _ => {}, // Handle other states here
        })?;
        Ok(())
    }
}