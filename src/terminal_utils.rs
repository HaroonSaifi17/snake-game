use crossterm::{
    cursor, execute,
    style::{Color, Print, SetBackgroundColor},
    terminal::{self, ClearType},
    QueueableCommand,
};
use std::io:: Write;

pub fn initialize_terminal<W: Write>(stdout: &mut W) {
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, cursor::Hide).unwrap();
}

pub fn reset_terminal<W: Write>(stdout: &mut W, width: u16, height: u16) {
    stdout.queue(cursor::Show).unwrap();
    stdout.queue(cursor::MoveTo(width, height + 3)).unwrap();
    stdout.flush().unwrap();
    terminal::disable_raw_mode().unwrap();
}

pub fn clear_screen<W: Write>(stdout: &mut W) {
    stdout
        .queue(SetBackgroundColor(Color::Rgb {
            r: 30,
            g: 30,
            b: 30,
        }))
        .unwrap();
    stdout.queue(terminal::Clear(ClearType::All)).unwrap();
}

pub fn display_game_over<W: Write>(stdout: &mut W, width: u16, height: u16) {
    execute!(
        stdout,
        cursor::MoveTo(width / 2 - 5, height / 2),
        crossterm::style::Print("Game Over!"),
        cursor::Show
    )
    .unwrap();
}

pub fn get_terminal_size() -> (u16, u16) {
    let (width, height) = terminal::size().unwrap();
    if width % 2 != 0 {
        (width - 1, height - 3)
    } else {
        (width, height - 3)
    }
}
pub fn display_score<W: Write>(stdout: &mut W, score: u16, _width: u16, _height: u16) {
    for i in _height.._height + 3 {
        stdout.queue(cursor::MoveTo(0, i)).unwrap();
        stdout
            .queue(Print(" ".repeat(terminal::size().unwrap().0.into())))
            .unwrap();
    }
    stdout.queue(cursor::MoveTo(0, _height + 1)).unwrap();
    stdout.queue(Print(format!("Score: {}", score))).unwrap();
    stdout
        .queue(cursor::MoveTo(_width - 14, _height + 1))
        .unwrap();
    stdout.queue(Print("Escape to exit")).unwrap();
}
