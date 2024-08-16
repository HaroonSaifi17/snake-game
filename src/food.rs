use crossterm::{cursor, QueueableCommand, style::{Color, Print, ResetColor, SetBackgroundColor}};
use std::io::Write;

pub struct Food {
    pub x: u16,
    pub y: u16,
}

impl Food {
    pub fn new(width: u16, height: u16) -> Self {
        let mut x = rand::random::<u16>() % width;
        let y = rand::random::<u16>() % height;

        if x % 2 != 0 {
            x -= 1;
        }

        Self { x, y }
    }

    pub fn draw<W: Write>(&self, stdout: &mut W) {
        stdout.queue(cursor::MoveTo(self.x, self.y)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Rgb { r: 0, g: 255, b: 0 })).unwrap();
        stdout.queue(Print("  ")).unwrap();
        stdout.queue(ResetColor).unwrap();
    }
}

