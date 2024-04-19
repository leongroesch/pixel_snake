use crate::definitions::*;
use crate::snake::*;
use pixel_engine::prelude::*;

use rand::Rng;
use std::time::Duration;

pub struct Food {
    rectangle: Rectangle,
}

impl Food {
    fn new(x: u8, y: u8) -> Self {
        let rectangle = Rectangle::builder()
            .position(x, y)
            .size(1, 1)
            .color(Color::from_u32(0x0000FF))
            .build();
        Self { rectangle }
    }
}

pub struct FoodEngine {
    food_list: Vec<Food>,
    spawn_timer: FrameRate,
}

impl FoodEngine {
    pub fn new(spawn_interval: Duration) -> Self {
        Self {
            food_list: Vec::new(),
            spawn_timer: FrameRate::new(spawn_interval),
        }
    }

    pub fn update(&mut self, snake: &Snake) {
        if self.spawn_timer.next_frame() {
            let mut rng = rand::thread_rng();
            loop {
                let x: u8 = rng.gen_range(0..=(WIDTH - 1) / 2) * 2 + 1;
                let y: u8 = rng.gen_range(0..=(HEIGHT - 1) / 2) * 2 + 1;
                if !snake.occupies_field(x, y) {
                    self.food_list.push(Food::new(x, y));
                    break;
                }
            }
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        for food in &self.food_list {
            frame.draw(&food.rectangle);
        }
    }
}
