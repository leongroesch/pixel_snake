use crate::definitions::*;
use pixel_engine::prelude::*;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Head {
    pub rectangle: Rectangle,
    pub direction: Direction,
}

impl Head {
    pub fn new() -> Self {
        let rectangle = Rectangle::builder()
            .position(1, 1)
            .size(1, 1)
            .color(Color::from_u32(0xFF))
            .build();
        Self {
            rectangle,
            direction: Direction::Right,
        }
    }
    pub fn update(&mut self) {
        let (mut x, mut y) = move_point((self.rectangle.x, self.rectangle.y), self.direction);
        (x, y) = move_point((x, y), self.direction);
        self.rectangle.x = x;
        self.rectangle.y = y;        
    }
}

pub struct Snake {
    pub elements: Vec<Rectangle>,
    pub append: bool,
}

impl Snake {
    pub fn update(&mut self, head: &Head) {
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
            let new_element = head.rectangle.clone();
            self.elements.push(new_element);
            self.append = false;
        }
    }
}

pub fn move_point(point: (u8, u8), direction: Direction) -> (u8, u8) {
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