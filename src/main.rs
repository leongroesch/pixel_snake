mod definitions;
mod food;
mod snake;

use definitions::*;
use food::*;
use snake::*;

use pixel_engine::prelude::*;

use std::time::Duration;

const INITIAL_SNAKE_POSITION: (u8, u8) = (1, 1);
fn main() {
    let mut frame = Frame::new(Color::from_u32(0x0), WIDTH, HEIGHT);

    let mut snake = Snake::new(INITIAL_SNAKE_POSITION.0, INITIAL_SNAKE_POSITION.1);
    let mut food_engine = FoodEngine::new(Duration::from_secs(2));

    let mut transmitter = Transmitter::new_retry().unwrap();

    let mut keyboard = Keyboard::new();
    let mut frame_rate = FrameRate::new(FRAME_DELTA);

    let mut running = true;
    while running {
        keyboard.update();

        if frame_rate.next_frame() {
            for key in keyboard.consume_keys() {
                match key {
                    Keycode::Left => {
                        snake.set_direction(Direction::Left);
                    }
                    Keycode::Right => {
                        snake.set_direction(Direction::Right);
                    }
                    Keycode::Up => {
                        snake.set_direction(Direction::Up);
                    }
                    Keycode::Down => {
                        snake.set_direction(Direction::Down);
                    }
                    Keycode::Space => {
                        snake.grow();
                    }
                    Keycode::R => {
                        snake = Snake::new(INITIAL_SNAKE_POSITION.0, INITIAL_SNAKE_POSITION.1);
                    }
                    Keycode::Escape => {
                        running = false;
                    }
                    _ => {}
                }
            }
            frame.clear();
            if !snake.game_over {
                food_engine.update(&snake);
                snake.update();
            }

            let snake_head = snake.get_head_position();
            if food_engine.try_eat_position(snake_head.0, snake_head.1) {
                snake.grow()
            }

            food_engine.draw(&mut frame);
            snake.draw(&mut frame);

            transmitter.transmit_frame(&frame).unwrap();
        }
    }
}
