use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self},
    style::{self, Stylize},
    terminal::{self},
};
use std::io::{self, Write};

use rand::Rng;

pub const FRAME_WIDTH: u16 = 21;
pub const FRAME_HEIGHT: u16 = 7;

pub fn start() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    print_frame()?;
    terminal::enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;

    Ok(())
}

pub fn end() -> io::Result<()> {
    let mut stdout = io::stdout();
    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;
    stdout.execute(cursor::MoveTo(0, FRAME_HEIGHT + 2))?;
    stdout.flush()?;
    Ok(())
}

pub fn locate(x: i32, y: i32, text: &str) -> io::Result<()> {
    // Check that coordinates are within the frame
    if x < 1 || y < 1 || (x as u16) > FRAME_WIDTH || (y as u16) > FRAME_HEIGHT {
        end()?;
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "coordinates outside frame",
        ));
    }

    // Compute available space from x to the right frame edge and clip text
    let available_columns = (FRAME_WIDTH as i32) - x + 1; // inclusive of current column
    if available_columns <= 0 {
        return Ok(()); // nothing to draw
    }

    let clipped: String = text.chars().take(available_columns as usize).collect();

    let mut stdout = io::stdout();
    stdout.execute(cursor::MoveTo(x as u16, y as u16))?;
    if !clipped.is_empty() {
        stdout.queue(style::PrintStyledContent(clipped.stylize()))?;
    }
    stdout.flush()?;
    Ok(())
}

pub fn getkey() -> io::Result<String> {
    loop {
        if let crossterm::event::Event::Key(event) = crossterm::event::read()? {
            match event.code {
                crossterm::event::KeyCode::Char(c) => return Ok(c.to_string()),
                crossterm::event::KeyCode::Left => return Ok("Left".to_string()),
                crossterm::event::KeyCode::Right => return Ok("Right".to_string()),
                crossterm::event::KeyCode::Up => return Ok("Up".to_string()),
                crossterm::event::KeyCode::Down => return Ok("Down".to_string()),
                _ => continue,
            }
        }
    }
}

pub fn ran_int(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}

pub fn pause() -> io::Result<()> {
    loop {
        if let crossterm::event::Event::Key(event) = crossterm::event::read()? {
            if let crossterm::event::KeyCode::Enter = event.code {
                return Ok(());
            }
        }
    }
}

pub fn clr_text() -> io::Result<()> {
    let mut stdout = io::stdout();
    for row in 1..=FRAME_HEIGHT {
        stdout.execute(cursor::MoveTo(1, row))?;
        stdout.queue(style::PrintStyledContent(
            " ".repeat(FRAME_WIDTH as usize).stylize(),
        ))?;
    }
    stdout.flush()?;
    Ok(())
}

pub fn print_frame() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.queue(style::PrintStyledContent(
        "┌────── EBASIC  ──────┐\n".yellow(),
    ))?;
    for _ in 0..FRAME_HEIGHT {
        let interior = format!("│{}│\n", " ".repeat(FRAME_WIDTH as usize));
        stdout.queue(style::PrintStyledContent(interior.yellow()))?;
    }
    // Keep the bottom border matching the interior width
    let bottom = format!("└{}┘\n", "─".repeat(FRAME_WIDTH as usize));
    stdout.queue(style::PrintStyledContent(bottom.yellow()))?;

    Ok(())
}
