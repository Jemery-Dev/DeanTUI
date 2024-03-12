use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Style, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::app::App;

impl App<'_> {
    pub fn ui_main(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.size());

        // Premier bloc avec un titre différent
        let welcome_block = Block::default()
            .title("Welcome to Dean TUI")
            .borders(Borders::ALL)
            .style(Style::default());

        let welcome_paragraph = Paragraph::new(Text::styled(
            "Press enter to make a new project",
            Style::default().fg(Color::Magenta),
        ))
            .block(welcome_block);

        let import_block = Block::default()
            .title("Import File")
            .borders(Borders::ALL)
            .style(Style::default());

        let import_paragraph = Paragraph::new(Text::styled(
            "Press I to import a file",
            Style::default().fg(Color::Magenta),
        ))
            .block(import_block);

        frame.render_widget(welcome_paragraph, chunks[0]);
        frame.render_widget(import_paragraph, chunks[1]);
    }
}
