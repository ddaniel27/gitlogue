use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct FileTreePane;

impl FileTreePane {
    pub fn render(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("File Tree")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let content = Paragraph::new(vec![
            Line::from("src/"),
            Line::from("  main.rs"),
            Line::from("  git.rs"),
            Line::from("  ui.rs"),
            Line::from("  panes/"),
            Line::from("    file_tree.rs"),
            Line::from("    editor.rs"),
            Line::from("    terminal.rs"),
            Line::from("    status_bar.rs"),
            Line::from("Cargo.toml"),
            Line::from("docs/"),
            Line::from("  specification.md"),
        ])
        .block(block);

        f.render_widget(content, area);
    }
}
