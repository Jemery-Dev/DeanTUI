use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Style, Text};
use ratatui::style::Color;
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use crate::app::app::App;
use crate::models::{tablature, note, time_signature};
use ratatui::style::palette::tailwind::SLATE;

impl App<'_> {
    pub fn ui_newproject(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Length(3), // Project Name
                Constraint::Length(3) //
            ])
            .split(frame.size());

        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "Make a new Project",
            Style::default().fg(SLATE.c900),
        ))
            .block(title_block);

        frame.render_widget(title, chunks[0]);

        let project_name_input = Paragraph::new(Text::raw(
            self.tab.as_ref().map_or("", |t| &t.name)
        ))
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        frame.render_widget(project_name_input, chunks[1]);
    }
}
