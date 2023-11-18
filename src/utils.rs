use nannou::{
    color::{
        encoding::{Linear, Srgb},
        rgb::Rgb,
    },
    prelude::*,
};

pub fn convert_to_lsrgb(r: u32, g: u32, b: u32) -> Rgb<Linear<Srgb>> {
    let lin_r: f32 = r as f32 / 255 as f32;
    let lin_g: f32 = g as f32 / 255 as f32;
    let lin_b: f32 = b as f32 / 255 as f32;

    lin_srgb(lin_r, lin_g, lin_b)
}
