use std::marker::PhantomData;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::disable_raw_mode;
use ratatui::{Frame, Terminal};
use ratatui::backend::Backend;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders, BorderType, Paragraph};

use app::app_states::AppState;

use crate::app;
use crate::models::tablature::Tablature;

#[derive(Debug, Copy, Clone)]
pub struct App<'a> {
    pub tab: Option<Tablature>,
    pub should_quit: bool,
    pub state: AppState,
    phantom: PhantomData<&'a ()>,
    // Add more vars after
}

impl App<'_> {
    pub fn new<'a>() -> App<'a> {
        App {
            tab: None,
            should_quit: false,
            state: AppState::CreatingNewTab,
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
        terminal.draw(|frame | self.ui(frame))?;
        Ok(())
    }

    pub fn ui(&mut self, frame: &mut Frame) {
        let block = Block::default()
            .title("Dean TUI")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightMagenta))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black));

        let greeting = Paragraph::new("Dean TUI Affichage ou quoi!").block(block);
        frame.render_widget(greeting, frame.size());
    }



    pub async fn handle_events(&mut self) {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match self.state {
                    AppState::Normal => match key.code {
                        KeyCode::Char('q') => self.should_quit = true,

                        //Other keycode

                        _ => {}
                    }
                    //Other state of app
                    _ => {}
                }
            }
        }
    }
}