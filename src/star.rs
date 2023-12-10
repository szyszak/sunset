use nannou::prelude::*;
use rand::Rng;

use crate::{constants::CONSTANTS, enums::StarOrigin};

#[derive(Debug)]
pub struct Star {
    pub position: Vec2,
    pub size: f32,
}

impl Star {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(
            -(CONSTANTS.WINDOW_WIDTH as f32 / 2.0)..(CONSTANTS.WINDOW_WIDTH as f32 / 2.0),
        );

        let y = rng.gen_range(0.0..(CONSTANTS.WINDOW_HEIGHT as f32 / 2.0));

        Star {
            position: vec2(x, y),
            size: rng.gen_range(1.0..4.0),
        }
    }

    pub fn update(&mut self) {
        self.position.x += CONSTANTS.STAR_VELOCITY;
        self.position.y += CONSTANTS.STAR_VELOCITY;
    }

    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();

        let mut position = vec2(-(CONSTANTS.WINDOW_WIDTH as f32 / 2.0), 0.0);

        let origin = StarOrigin::random();

        match origin {
            StarOrigin::Side => {
                position.y = rng.gen_range(0.0..(CONSTANTS.WINDOW_HEIGHT / 2) as f32);
            }
            StarOrigin::Bottom => {
                position.x = rng.gen_range(
                    -(CONSTANTS.WINDOW_WIDTH as f32 / 2.0)..(CONSTANTS.WINDOW_WIDTH as f32 / 2.0),
                );
            }
        }

        self.position = position;
        self.size = rng.gen_range(2.0..5.0);
    }

    pub fn is_out_of_bounds(&self) -> bool {
        if self.position.x - self.size > (CONSTANTS.WINDOW_WIDTH / 2) as f32
            || self.position.y - self.size > (CONSTANTS.WINDOW_HEIGHT / 2) as f32
        {
            return true;
        }

        false
    }
}
