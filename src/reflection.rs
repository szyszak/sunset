pub use crate::constants::CONSTANTS;
pub use crate::enums::Direction;
use crate::utils::calculate_alpha;
use nannou::prelude::*;
use rand::Rng;

#[derive(Debug)]
pub struct Reflection {
    pub width: f32,
    pub thickness: f32,
    pub starting_thickness: f32,
    pub is_blinking: bool,
    pub blinking_direction: Direction,
    pub position: Vec2,
    pub starting_x: f32,
    pub direction: Direction,
    pub velocity: f32,
    pub max_distance: f32,
    pub alpha: u8,
}

impl Reflection {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let width: f32 = rng.gen_range(10.0..30.0);
        let thickness: f32 = rng.gen_range(2.0..4.0);
        let position: Vec2 = Vec2::new(
            rng.gen_range(
                CONSTANTS.SUN_POSITION.x - CONSTANTS.REFLECTION_MAX_OFFSET
                    ..CONSTANTS.SUN_POSITION.x + CONSTANTS.REFLECTION_MAX_OFFSET,
            ),
            -rng.gen_range(0.0..(CONSTANTS.WINDOW_HEIGHT / 2) as f32),
        );
        let velocity = rng.gen_range(0.05..0.15);
        let max_distance = rng.gen_range(3.0..7.0);
        let alpha = calculate_alpha(CONSTANTS.SUN_POSITION.x, position.x);

        Reflection {
            width,
            thickness,
            starting_thickness: thickness,
            is_blinking: false,
            blinking_direction: Direction::Forwards,
            position,
            starting_x: position.x,
            direction: Direction::random(),
            velocity,
            max_distance,
            alpha,
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Forwards => {
                self.position.x += self.velocity;

                if self.position.x > self.starting_x + self.max_distance {
                    self.direction = Direction::Backwards
                }
            }
            Direction::Backwards => {
                self.position.x -= self.velocity;

                if self.position.x < self.starting_x - self.max_distance {
                    self.direction = Direction::Forwards
                }
            }
        }

        if self.is_blinking == true {
            self.blink();
        }
    }

    pub fn blink(&mut self) {
        match self.blinking_direction {
            Direction::Forwards => {
                self.thickness -= CONSTANTS.REFLECTION_BLINK_SPEED;

                if self.thickness <= 0.0 {
                    self.blinking_direction = Direction::Backwards;
                }
            }
            Direction::Backwards => {
                self.thickness += CONSTANTS.REFLECTION_BLINK_SPEED;

                if self.thickness >= self.starting_thickness {
                    self.thickness = self.starting_thickness;
                    self.blinking_direction = Direction::Forwards;
                    self.is_blinking = false;
                }
            }
        }
    }
}
