use crate::constants::CONSTANTS;
use crate::enums::Direction;
use crate::utils::calculate_distance;
use nannou::prelude::*;
use rand::Rng;

pub struct SunAfterimage {
    pub position: Vec2,
    pub velocity: Vec2,
    pub direction: Direction,
}

impl SunAfterimage {
    pub fn new() -> SunAfterimage {
        let mut rng = rand::thread_rng();

        let random_x_vel: f32 = rng.gen_range(
            -CONSTANTS.SUN_AFTERIMAGE_MAX_VELOCITY..=CONSTANTS.SUN_AFTERIMAGE_MAX_VELOCITY,
        );
        let random_y_vel: f32 = rng.gen_range(
            -CONSTANTS.SUN_AFTERIMAGE_MAX_VELOCITY..=CONSTANTS.SUN_AFTERIMAGE_MAX_VELOCITY,
        );

        SunAfterimage {
            position: vec2(CONSTANTS.SUN_POSITION.x, CONSTANTS.SUN_POSITION.y),
            velocity: vec2(random_x_vel, random_y_vel),
            direction: Direction::Forwards,
        }
    }

    pub fn update(&mut self) {
        let distance = calculate_distance(self.position, CONSTANTS.SUN_POSITION);

        match self.direction {
            Direction::Forwards => {
                self.position.x += self.velocity.x;
                self.position.y += self.velocity.y;

                if distance >= CONSTANTS.SUN_AFTERIMAGE_MAX_DISTANCE {
                    self.direction = Direction::Backwards;
                }
            }
            Direction::Backwards => {
                self.position.x -= self.velocity.x;
                self.position.y -= self.velocity.y;

                if distance <= 0.0 {
                    self.position.x = CONSTANTS.SUN_POSITION.x;
                    self.position.y = CONSTANTS.SUN_POSITION.y;

                    self.direction = Direction::Forwards;

                    let mut rng = rand::thread_rng();

                    let random_x_vel: f32 = rng.gen_range(
                        -CONSTANTS.SUN_AFTERIMAGE_MAX_VELOCITY
                            ..=CONSTANTS.SUN_AFTERIMAGE_MAX_VELOCITY,
                    );
                    let random_y_vel: f32 = rng.gen_range(
                        -CONSTANTS.SUN_AFTERIMAGE_MAX_VELOCITY
                            ..=CONSTANTS.SUN_AFTERIMAGE_MAX_VELOCITY,
                    );

                    self.velocity = vec2(random_x_vel, random_y_vel);
                }
            }
        }
    }
}
