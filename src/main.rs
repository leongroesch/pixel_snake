use pixel_engine::frame_data::frame::*;
use pixel_engine::frame_data::pixel::*;
use pixel_engine::frame_transfer::transmitter::*;
use pixel_engine::geometry::ellipse::*;
use pixel_engine::geometry::rectangle::*;
use pixel_engine::misc::frame_rate::*;
use pixel_engine::user_input::keyboard::*;

use std::time::Duration;

const WIDTH: u8 = 64;
const HEIGHT: u8 = 64;
const FRAME_DELTA: Duration = Duration::from_millis(125);

fn main() {
    let mut frame = Frame::new(Color::from_u32(0x0), WIDTH, HEIGHT);

    let mut rectangle = Rectangle::builder()
        .position(1, 1)
        .size(3, 3)
        .color(Color::from_u32(0xFF))
        .build();

    let mut circle = Ellipse::new((16.0, 16.0), 4.0, 4.0);

    let mut transmitter = Transmitter::new_retry().unwrap();

    let mut keyboard = Keyboard::new();
    let mut frame_rate = FrameRate::new(FRAME_DELTA);
    loop {
        keyboard.update();

        if frame_rate.next_frame() {
            for key in keyboard.consume_keys().into_iter() {
                match key {
                    Keycode::Left => {
                        rectangle.x -= 1;
                        circle.center.0 -= 1.;
                    }
                    Keycode::Right => {
                        rectangle.x += 1;
                        circle.center.0 += 1.;
                    }
                    Keycode::Up => {
                        rectangle.y -= 1;
                        circle.center.1 -= 1.;
                    }
                    Keycode::Down => {
                        rectangle.y += 1;
                        circle.center.1 += 1.;
                    }
                    _ => {}
                }
            }
            frame.clear();
            frame.draw(&rectangle);
            frame.draw(&circle);
            transmitter.transmit_frame(&frame).unwrap();
        }
    }
}
