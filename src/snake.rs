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
}

impl Snake {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            head: Head::new(x, y),
            tail: Tail::new(),
        }
    }

    pub fn update(&mut self) {
        self.head.update();
        self.tail.update(&self.head)
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
        if self.head.rectangle.x == x && self.head.rectangle.y == y {
            return true;
        }
        for element in &self.tail.elements {
            if element.x == x && element.y == y {
                return true;
            }
        }

        false
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.head.direction = direction;
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
    fn update(&mut self, head: &Head) {
        if self.grow {
            let new_element = head.rectangle.clone();
            self.elements.push(new_element);
            self.grow = false;
        }
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
