use ratatui::Frame;
use crate::app::app::App;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::app_states::AppState;
impl App<'_> {
    pub fn ui_error(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.size());

        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "ERROR !!!!!!",
            Style::default().fg(Color::Red),
        ))
            .block(title_block);

        frame.render_widget(title, chunks[0]);
    }
}