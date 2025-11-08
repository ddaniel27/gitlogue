use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct TerminalPane;

impl TerminalPane {
    pub fn render(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("Terminal")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        let content = Paragraph::new(vec![
            Line::from("$ git log --oneline"),
            Line::from("8ec9a9c Merge pull request #14"),
            Line::from("7f5db95 feat: add full file content extraction"),
        ])
        .block(block);

        f.render_widget(content, area);
    }
}
