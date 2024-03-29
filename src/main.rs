use pixel_engine::prelude::*;

use std::time::Duration;

const WIDTH: u8 = 64;
const HEIGHT: u8 = 64;
const FRAME_DELTA: Duration = Duration::from_millis(500);

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn inverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct Head {
    rectangle: Rectangle,
    direction: Direction,
}

impl Head {
    fn new() -> Self {
        let mut rectangle = Rectangle::builder()
            .position(1, 1)
            .size(1, 1)
            .color(Color::from_u32(0xFF))
            .build();
        Self {
            rectangle,
            direction: Direction::Right,
        }
    }
    fn update(&mut self) {
        let (mut x, mut y) = move_point((self.rectangle.x, self.rectangle.y), self.direction);
        (x, y) = move_point((x, y), self.direction);
        self.rectangle.x = x;
        self.rectangle.y = y;
    }
}

struct Snake {
    elements: Vec<Rectangle>,
    append: bool,
}

impl Snake {
    fn update(&mut self, head: &Head) {
        if self.elements.len() > 1 {
            for i in 0..(self.elements.len() - 1) {
                let right = self.elements[i + 1].clone();
                let left = &mut self.elements[i];
                left.x = right.x;
                left.y = right.y;
            }
        }
        if let Some(element) = self.elements.last_mut() {
            element.x = head.rectangle.x;
            element.y = head.rectangle.y;
        }
        if self.append {
            let mut new_element = head.rectangle.clone();
            // let (mut x, mut y) = move_point((new_element.x, new_element.y), head.direction.inverse());
            // (x, y) = move_point((x, y), head.direction.inverse());
            // new_element.x = x;
            // new_element.y = y;
            println!("new_element: ({}|{})", new_element.x, new_element.y);
            println!("head: ({}|{})", head.rectangle.x, head.rectangle.y);
            self.elements.push(new_element);
            self.append = false;
        }
    }
}

fn move_point(point: (u8, u8), direction: Direction) -> (u8, u8) {
    let mut result = point;
    match direction {
        Direction::Up => result.1 = if point.1 == 0 { HEIGHT } else { point.1 - 1 },
        Direction::Down => {
            result.1 = (point.1 + 1) % HEIGHT;
        }
        Direction::Left => result.0 = if point.0 == 0 { WIDTH } else { point.0 - 1 },
        Direction::Right => {
            result.0 = (result.0 + 1) % WIDTH;
        }
    }
    result
}

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
            snake.update(&head);
            head.update();
            frame.draw(&(head.rectangle));
            for element in &snake.elements {
                frame.draw(element);
            }
            transmitter.transmit_frame(&frame).unwrap();
        }
    }
}
