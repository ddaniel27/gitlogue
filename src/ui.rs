use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};
use std::io;

use crate::panes::{EditorPane, FileTreePane, StatusBarPane, TerminalPane};

pub struct UI {
    should_quit: bool,
    file_tree: FileTreePane,
    editor: EditorPane,
    terminal: TerminalPane,
    status_bar: StatusBarPane,
}

impl UI {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            file_tree: FileTreePane,
            editor: EditorPane,
            terminal: TerminalPane,
            status_bar: StatusBarPane,
        }
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

        self.file_tree.render(f, content_layout[0]);
        self.editor.render(f, right_layout[0]);
        self.terminal.render(f, right_layout[1]);
        self.status_bar.render(f, main_layout[1]);
    }
}
