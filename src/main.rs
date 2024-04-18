mod snake;
mod definitions;

use definitions::*;
use snake::*;

use pixel_engine::prelude::*;

fn main() {
    let mut frame = Frame::new(Color::from_u32(0x0), WIDTH, HEIGHT);

    let mut head = Head::new();
    let mut snake = Snake {
        elements: Vec::new(),
        append: false,
    };

    let mut transmitter = Transmitter::new_retry().unwrap();

    let mut keyboard = Keyboard::new();
    let mut frame_rate = FrameRate::new(FRAME_DELTA);
    loop {
        keyboard.update();

        if frame_rate.next_frame() {
            if let Some(key) = keyboard.consume_keys().last() {
                match key {
                    Keycode::Left => {
                        head.direction = Direction::Left;
                    }
                    Keycode::Right => {
                        head.direction = Direction::Right;
                    }
                    Keycode::Up => {
                        head.direction = Direction::Up;
                    }
                    Keycode::Down => {
                        head.direction = Direction::Down;
                    }
                    Keycode::Space => {
                        snake.append = true;
                    }
                    _ => {}
                }
            }
            frame.clear();
            head.update();
            snake.update(&head);
            frame.draw(&(head.rectangle));
            for element in &snake.elements {
                frame.draw(element);
            }
            transmitter.transmit_frame(&frame).unwrap();
        }
    }
}
