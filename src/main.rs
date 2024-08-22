mod snake;
mod food;
mod input;
mod terminal_utils;

use std::io::{self, Write};

fn main() {
    let mut score = 0;

    let (width, height) = terminal_utils::get_terminal_size();

    let mut stdout = io::stdout();
    terminal_utils::initialize_terminal(&mut stdout);

    let mut snake = snake::Snake::new(width, height);
    let mut food = food::Food::new(width, height);

    terminal_utils::clear_screen(&mut stdout);

    loop {
        input::handle_input(&mut snake);
        snake.move_forward(width, height);

        snake.draw(&mut stdout);
        food.draw(&mut stdout);
        terminal_utils::display_score(&mut stdout, score, width, height);

        stdout.flush().unwrap();

        if snake.has_collided(width, height) {
            terminal_utils::display_game_over(&mut stdout, width, height);
            break;
        }

        if snake.has_eaten_food(&food , &mut stdout) {
            snake.grow();
            food = food::Food::new(width, height);
            score += 1;
        }
    }

    terminal_utils::reset_terminal(&mut stdout, width, height);
}
