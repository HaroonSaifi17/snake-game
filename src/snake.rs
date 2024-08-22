use crossterm::{
    cursor,
    style::{Color, Print, SetBackgroundColor},
    QueueableCommand,
};
use std::io::Write;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: Vec<(u16, u16)>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(width: u16, height: u16) -> Self {
        let mut body = Vec::new();
        let mut x = width / 2;
        if x % 2 != 0 {
            x -= 1;
        }
        body.push((x / 2, height / 2));
        body.push(((x / 2) + 2, height / 2));
        body.push(((x / 2) + 4, height / 2));
        body.push(((x / 2) + 6, height / 2));

        Self {
            body,
            direction: Direction::Left,
        }
    }

    pub fn move_forward(&mut self, width: u16, height: u16) {
        for i in (1..self.body.len()).rev() {
            self.body[i] = self.body[i - 1];
        }

        match self.direction {
            Direction::Up => {
                if self.body[0].1 > 0 {
                    self.body[0].1 -= 1;
                }
            }
            Direction::Down => {
                if self.body[0].1 < height - 1 {
                    self.body[0].1 += 1;
                }
            }
            Direction::Right => {
                if self.body[0].0 < width - 2 {
                    self.body[0].0 += 2;
                }
            }
            Direction::Left => {
                if self.body[0].0 > 1 {
                    self.body[0].0 -= 2;
                }
            }
        }
    }

    pub fn draw<W: Write>(&self, stdout: &mut W) {
        stdout
            .queue(SetBackgroundColor(Color::Rgb { r: 255, g: 0, b: 0 }))
            .unwrap();
        for segment in &self.body {
            stdout.queue(cursor::MoveTo(segment.0, segment.1)).unwrap();
            stdout.queue(Print("  ")).unwrap();
        }
        stdout.queue(cursor::MoveTo(self.body[self.body.len()-1].0, self.body[self.body.len()-1].1)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Rgb { r: 30, g: 30, b: 30 })).unwrap();
        stdout.queue(Print("  ")).unwrap();
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    pub fn has_collided(&self, width: u16, height: u16) -> bool {
        let head = self.body[0];
        head.0 == 0
            || head.0 >= width
            || head.1 == 0
            || head.1 >= height
            || self.body[1..self.body.len() - 1].contains(&head)
    }
    pub fn has_eaten_food<W: Write>(&self, food: &crate::food::Food, stdout: &mut W ) -> bool {
        let bool = self.body[0].0 == food.x && self.body[0].1 == food.y;
        if bool{
            stdout.queue(cursor::MoveTo(food.x, food.y)).unwrap();
            stdout.queue(Print("  ")).unwrap();
        }
        bool
    }

    pub fn grow(&mut self) {
        let last = *self.body.last().unwrap();
        let second_last = self.body[self.body.len() - 2];

        if last.0 == second_last.0 {
            if last.1 > second_last.1 {
                self.body.push((last.0, last.1 + 1));
            } else {
                self.body.push((last.0, last.1 - 1));
            }
        } else {
            if last.0 > second_last.0 {
                self.body.push((last.0 + 2, last.1));
            } else {
                self.body.push((last.0 - 2, last.1));
            }
        }
    }
}
