pub use crate::constants::CONSTANTS;
pub use crate::enums::Direction;
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
            position: Vec2::new(CONSTANTS.SUN_POSITION.x, CONSTANTS.SUN_POSITION.y),
            velocity: Vec2::new(random_x_vel, random_y_vel),
            direction: Direction::Forwards,
        }
    }
}
