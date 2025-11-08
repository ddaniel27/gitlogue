use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct StatusBarPane;

impl StatusBarPane {
    pub fn render(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));

        let status_text = vec![Line::from(
            "git-logue v0.1.0 | Commit: abc123 | Author: User | Press 'q' to quit",
        )];

        let content = Paragraph::new(status_text).block(block);

        f.render_widget(content, area);
    }
}
