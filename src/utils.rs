use nannou::{
    color::{
        encoding::{Linear, Srgb},
        rgb::Rgb,
    },
    prelude::*,
};

use crate::constants::CONSTANTS;
use rand::Rng;
use std::cmp;

pub fn convert_to_lsrgb(r: u32, g: u32, b: u32) -> Rgb<Linear<Srgb>> {
    let lin_r: f32 = r as f32 / 255 as f32;
    let lin_g: f32 = g as f32 / 255 as f32;
    let lin_b: f32 = b as f32 / 255 as f32;

    lin_srgb(lin_r, lin_g, lin_b)
}

pub fn calculate_distance(pos: Vec2, origin: Vec2) -> f32 {
    let distance = ((pos[0] - origin[0]).powi(2) + (pos[1] - origin[1]).powi(2)).sqrt();

    distance
}

pub fn calculate_alpha(origin_x: f32, object_x: f32) -> u8 {
    let distance = f32::abs(object_x - origin_x);

    let alpha = cmp::min(
        (CONSTANTS.ALPHA_DISTANCE_SCALE / (distance + 1.0)).round() as u8,
        255,
    );

    alpha
}

pub fn random_with_probability(probability_percent: u32) -> bool {
    let mut rng = rand::thread_rng();

    let random_number = rng.gen_range(0..=99);

    let result = random_number < probability_percent;

    result
}
