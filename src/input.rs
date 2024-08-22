use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn handle_input(snake: &mut crate::snake::Snake) {
    if event::poll(Duration::from_millis(1000)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match (key_event.code, &snake.direction) {
                (KeyCode::Esc, _) => {
                    std::process::exit(0);
                }
                (KeyCode::Up, crate::snake::Direction::Right | crate::snake::Direction::Left) => {
                    snake.change_direction(crate::snake::Direction::Up);
                }
                (KeyCode::Right, crate::snake::Direction::Up | crate::snake::Direction::Down) => {
                    snake.change_direction(crate::snake::Direction::Right);
                }
                (KeyCode::Down, crate::snake::Direction::Right | crate::snake::Direction::Left) => {
                    snake.change_direction(crate::snake::Direction::Down);
                }
                (KeyCode::Left, crate::snake::Direction::Up | crate::snake::Direction::Down) => {
                    snake.change_direction(crate::snake::Direction::Left);
                }
                _ => {}
            }
        }
    }
}

