use lazy_static::lazy_static;
use nannou::prelude::*;

#[allow(non_snake_case)]
pub struct Constants {
    pub WINDOW_HEIGHT: u32,
    pub WINDOW_WIDTH: u32,
    pub BACKGROUND_STEP: usize,
    pub SUN_XY: Vec2,
    pub SUN_AFTERIMAGES_COUNT: usize,
    pub SUN_AFTERIMAGE_MAX_VALOCITY: f32,
    pub SUN_AFTERIMAGE_MAX_DISTANCE: f32,
}

impl Constants {
    pub fn new() -> Self {
        Constants {
            WINDOW_HEIGHT: 800,
            WINDOW_WIDTH: 500,
            BACKGROUND_STEP: 10,
            SUN_XY: Vec2::new(-100.0, 200.0),
            SUN_AFTERIMAGES_COUNT: 4,
            SUN_AFTERIMAGE_MAX_VALOCITY: 0.3,
            SUN_AFTERIMAGE_MAX_DISTANCE: 5.0,
        }
    }
}

lazy_static! {
    pub static ref CONSTANTS: Constants = Constants::new();
}
