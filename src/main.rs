mod definitions;
mod snake;

use definitions::*;
use snake::*;

use pixel_engine::prelude::*;

fn main() {
    let mut frame = Frame::new(Color::from_u32(0x0), WIDTH, HEIGHT);

    let mut snake = Snake::new(1, 1);

    let mut transmitter = Transmitter::new_retry().unwrap();

    let mut keyboard = Keyboard::new();
    let mut frame_rate = FrameRate::new(FRAME_DELTA);
    loop {
        keyboard.update();

        if frame_rate.next_frame() {
            if let Some(key) = keyboard.consume_keys().last() {
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
                    _ => {}
                }
            }
            }
            frame.clear();
            snake.update();
            snake.draw(&mut frame);

            transmitter.transmit_frame(&frame).unwrap();
        }
    }
}
