use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct EditorPane;

impl EditorPane {
    pub fn render(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("Editor")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));

        let content = Paragraph::new(vec![
            Line::from("fn main() -> Result<()> {"),
            Line::from("    println!(\"git-logue v0.1.0\");"),
            Line::from("    Ok(())"),
            Line::from("}"),
        ])
        .block(block);

        f.render_widget(content, area);
    }
}
