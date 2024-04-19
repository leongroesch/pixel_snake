use crate::definitions::*;
use pixel_engine::prelude::*;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    head: Head,
    tail: Tail,
    pub game_over: bool,
}

impl Snake {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            head: Head::new(x, y),
            tail: Tail::new(),
            game_over: false,
        }
    }

    pub fn update(&mut self) {
        let old_head = self.head.rectangle.clone();
        self.head.update();

        let (x, y) = (self.head.rectangle.x, self.head.rectangle.y);
        if self.tail.occupies_field(x, y) {
            self.game_over = true;
            self.head.rectangle = old_head;
            println!("Game over!\t Score: {}", self.tail.elements.len());
        } else {
            self.tail.update(&old_head)
        }
    }

    pub fn grow(&mut self) {
        self.tail.grow = true
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.draw(&(self.head.rectangle));
        for element in &self.tail.elements {
            frame.draw(element);
        }
    }

    pub fn occupies_field(&self, x: u8, y: u8) -> bool {
        self.head.occupies_field(x, y) || self.tail.occupies_field(x, y)
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.head.direction = direction;
    }

    pub fn get_head_position(&self) -> (u8, u8) {
        (self.head.rectangle.x, self.head.rectangle.y)
    }
}

struct Head {
    rectangle: Rectangle,
    direction: Direction,
}

impl Head {
    fn new(x: u8, y: u8) -> Self {
        let rectangle = Rectangle::builder()
            .position(x, y)
            .size(1, 1)
            .color(Color::from_u32(0xFF0000))
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

    fn occupies_field(&self, x: u8, y: u8) -> bool {
        self.rectangle.x == x && self.rectangle.y == y
    }
}

pub struct Tail {
    elements: Vec<Rectangle>,
    grow: bool,
}

impl Tail {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            grow: false,
        }
    }
    fn update(&mut self, head: &Rectangle) {
        if self.elements.len() < 1 {
            if self.grow {
                self.elements.push(head.clone());
            }
        } else {
            let old_last = self.elements.last().unwrap().clone();

            for idx in (1..self.elements.len()).rev() {
                let left = self.elements[idx - 1].clone();
                self.elements[idx].x = left.x;
                self.elements[idx].y = left.y;
            }

            let first = self.elements.first_mut().unwrap();
            first.x = head.x;
            first.y = head.y;

            if self.grow {
                self.elements.push(old_last);
            }
        }
        self.grow = false
    }

    fn occupies_field(&self, x: u8, y: u8) -> bool {
        for element in &self.elements {
            if element.x == x && element.y == y {
                return true;
            }
        }
        false
    }
}

fn move_point(point: (u8, u8), direction: Direction) -> (u8, u8) {
    let mut result = point;
    match direction {
        Direction::Up => result.1 = if point.1 == 0 { HEIGHT -1 } else { point.1 - 1 },
        Direction::Down => {
            result.1 = (point.1 + 1) % HEIGHT;
        }
        Direction::Left => result.0 = if point.0 == 0 { WIDTH -1 } else { point.0 - 1 },
        Direction::Right => {
            result.0 = (result.0 + 1) % WIDTH;
        }
    }
    result
}
