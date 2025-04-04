use crossterm::{event, execute, terminal};
use ratatui::{prelude::*, widgets::*};
use std::{io, time::Duration};
use sysinfo::System;

fn main() -> io::Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;

    let mut sys = System::new_all();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    loop {
        sys.refresh_all();

        terminal.draw(|frame| {
            let area = frame.area();
            let block = Block::default().borders(Borders::ALL).title("RTOP");
            let text = vec![
                Line::from(format!("Total Memory: {} GB", sys.total_memory())),
                Line::from(format!("Used Memory: {} GB", sys.used_memory())),
            ];
            let paragraph = Paragraph::new(text).block(block);
            frame.render_widget(paragraph, area);
        })?;

        if event::poll(Duration::from_millis(1000))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Restore terminal
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
