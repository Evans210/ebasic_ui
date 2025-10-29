use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{self},
    style::{self, Stylize},
    terminal::{self},
};
use std::io::{self, Write};

use rand::Rng;

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
    stdout.execute(cursor::MoveTo(0, 9))?;
    stdout.flush()?;
    Ok(())
}

pub fn locate(x: u16, y: u16, text: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(cursor::MoveTo(x, y))?;
    stdout.queue(style::PrintStyledContent(text.stylize()))?;
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
    for row in 1..=7 {
        stdout.execute(cursor::MoveTo(1, row))?;
        stdout.queue(style::PrintStyledContent(" ".repeat(21).stylize()))?;
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
    for _ in 0..7 {
        stdout.queue(style::PrintStyledContent(
            "│                     │\n".yellow(),
        ))?;
    }
    stdout.queue(style::PrintStyledContent(
        "└─────────────────────┘\n".yellow(),
    ))?;

    Ok(())
}
