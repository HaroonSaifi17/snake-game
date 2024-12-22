use crate::terminal_utils;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration;

pub fn handle_input(snake: &mut crate::snake::Snake) {
    let (width, height) = terminal_utils::get_terminal_size();
    if event::poll(Duration::from_millis(200)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match (key_event.code, key_event.modifiers, &snake.direction) {
                (KeyCode::Esc, _, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL, _) => {
                    terminal_utils::reset_terminal(&mut std::io::stdout(), width, height);
                    std::process::exit(0);
                }
                (
                    KeyCode::Up,
                    _,
                    crate::snake::Direction::Right | crate::snake::Direction::Left,
                )
                | (
                    KeyCode::Char('k'),
                    _,
                    crate::snake::Direction::Right | crate::snake::Direction::Left,
                ) => {
                    snake.change_direction(crate::snake::Direction::Up);
                }
                (
                    KeyCode::Right,
                    _,
                    crate::snake::Direction::Up | crate::snake::Direction::Down,
                )
                | (
                    KeyCode::Char('l'),
                    _,
                    crate::snake::Direction::Up | crate::snake::Direction::Down,
                ) => {
                    snake.change_direction(crate::snake::Direction::Right);
                }
                (
                    KeyCode::Down,
                    _,
                    crate::snake::Direction::Right | crate::snake::Direction::Left,
                )
                | (
                    KeyCode::Char('j'),
                    _,
                    crate::snake::Direction::Right | crate::snake::Direction::Left,
                ) => {
                    snake.change_direction(crate::snake::Direction::Down);
                }
                (KeyCode::Left, _, crate::snake::Direction::Up | crate::snake::Direction::Down)
                | (
                    KeyCode::Char('h'),
                    _,
                    crate::snake::Direction::Up | crate::snake::Direction::Down,
                ) => {
                    snake.change_direction(crate::snake::Direction::Left);
                }
                _ => {}
            }
        }
    }
}
