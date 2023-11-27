use lazy_static::lazy_static;
use nannou::prelude::*;

#[allow(non_snake_case)]
pub struct Constants {
    pub WINDOW_HEIGHT: u32,
    pub WINDOW_WIDTH: u32,
    pub BACKGROUND_STEP: usize,
    pub SUN_POSITION: Vec2,
    pub SUN_AFTERIMAGES_COUNT: usize,
    pub SUN_AFTERIMAGE_MAX_VELOCITY: f32,
    pub SUN_AFTERIMAGE_MAX_DISTANCE: f32,
    pub REFLECTION_LINES_COUNT: usize,
    pub REFLECTION_MAX_OFFSET: f32,
    pub REFLECTION_BLINK_CHANCE: u32,
    pub ALPHA_DISTANCE_SCALE: f32,
    pub REFLECTION_BLINK_SPEED: f32,
}

impl Constants {
    pub fn new() -> Self {
        Constants {
            WINDOW_HEIGHT: 800,
            WINDOW_WIDTH: 500,
            BACKGROUND_STEP: 10,
            SUN_POSITION: Vec2::new(-100.0, 200.0),
            SUN_AFTERIMAGES_COUNT: 4,
            SUN_AFTERIMAGE_MAX_VELOCITY: 0.2,
            SUN_AFTERIMAGE_MAX_DISTANCE: 5.0,
            REFLECTION_LINES_COUNT: 200,
            REFLECTION_MAX_OFFSET: 80.0,
            REFLECTION_BLINK_CHANCE: 1,
            ALPHA_DISTANCE_SCALE: 8000.0,
            REFLECTION_BLINK_SPEED: 0.3,
        }
    }
}

lazy_static! {
    pub static ref CONSTANTS: Constants = Constants::new();
}
