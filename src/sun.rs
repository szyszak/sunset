pub use crate::constants::CONSTANTS;
pub use crate::enums::Direction;
use nannou::prelude::*;
use rand::Rng;

pub struct SunAfterimage {
    pub x: f32,
    pub y: f32,
    pub velocity: Vec2,
    pub direction: Direction,
}

impl SunAfterimage {
    pub fn new() -> SunAfterimage {
        let mut rng = rand::thread_rng();

        let random_x_vel: f32 = rng.gen_range(
            -CONSTANTS.SUN_AFTERIMAGE_MAX_VALOCITY..=CONSTANTS.SUN_AFTERIMAGE_MAX_VALOCITY,
        );
        let random_y_vel: f32 = rng.gen_range(
            -CONSTANTS.SUN_AFTERIMAGE_MAX_VALOCITY..=CONSTANTS.SUN_AFTERIMAGE_MAX_VALOCITY,
        );

        SunAfterimage {
            x: CONSTANTS.SUN_XY[0],
            y: CONSTANTS.SUN_XY[1],
            velocity: Vec2::new(random_x_vel, random_y_vel),
            direction: Direction::Forwards,
        }
    }
}
