use crossterm::{event, execute, terminal};
use ratatui::{prelude::*, widgets::*};
use std::{io, time::Duration};
use sysinfo::System;

mod utils;

use utils::helpers::byte_to_readable_size;

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

            let block_memory = Block::default().borders(Borders::ALL).title("mem");
            let block_cores = Block::default().borders(Borders::ALL).title("cores");

            // Create a layout to split the screen vertically
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(area);

            // Memory
            let (total_readable, total_unit) = byte_to_readable_size(sys.total_memory());
            let (used_readable, used_unit) = byte_to_readable_size(sys.used_memory());

            let text_memory = vec![
                Line::from(format!(
                    "Total Memory: {:.2} {}",
                    total_readable, total_unit
                )),
                Line::from(format!("Used Memory:  {:.2} {}", used_readable, used_unit)),
            ];
            let paragraph_memory = Paragraph::new(text_memory).block(block_memory);
            frame.render_widget(paragraph_memory, layout[0]);

            // Cores
            let (total_readable, total_unit) = byte_to_readable_size(sys.total_memory());
            let (used_readable, used_unit) = byte_to_readable_size(sys.used_memory());

            let text_cores = vec![
                Line::from(format!(
                    "Total Memory: {:.2} {}",
                    total_readable, total_unit
                )),
                Line::from(format!("Used Memory:  {:.2} {}", used_readable, used_unit)),
            ];
            let paragraph_cores = Paragraph::new(text_cores).block(block_cores);
            frame.render_widget(paragraph_cores, layout[1]);
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
