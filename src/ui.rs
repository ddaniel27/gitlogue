use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;

pub struct UI {
    should_quit: bool,
}

impl UI {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_loop(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            self.should_quit = true;
                        }
                        _ => {}
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn render(&self, f: &mut Frame) {
        let size = f.area();

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),      // Main content area
                Constraint::Length(3),   // Status bar
            ])
            .split(size);

        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),  // Left side (file tree)
                Constraint::Percentage(70),  // Right side (editor + terminal)
            ])
            .split(main_layout[0]);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(80),  // Editor
                Constraint::Percentage(20),  // Terminal
            ])
            .split(content_layout[1]);

        self.render_file_tree(f, content_layout[0]);
        self.render_editor(f, right_layout[0]);
        self.render_terminal(f, right_layout[1]);
        self.render_status_bar(f, main_layout[1]);
    }

    fn render_file_tree(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title("File Tree")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let content = Paragraph::new(vec![
            Line::from("src/"),
            Line::from("  main.rs"),
            Line::from("  git.rs"),
            Line::from("  ui.rs"),
            Line::from("Cargo.toml"),
            Line::from("docs/"),
            Line::from("  specification.md"),
        ])
        .block(block);

        f.render_widget(content, area);
    }

    fn render_editor(&self, f: &mut Frame, area: Rect) {
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

    fn render_terminal(&self, f: &mut Frame, area: Rect) {
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

    fn render_status_bar(&self, f: &mut Frame, area: Rect) {
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
