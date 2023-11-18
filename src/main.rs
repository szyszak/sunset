use nannou::{color::Gradient, prelude::*};
// use rand::Rng;
mod utils;
use utils::convert_to_lsrgb;

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 500;
const BACKGROUND_STEP: usize = 10;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .view(view)
        .build()
        .unwrap();

    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();
    // let mut rng = rand::thread_rng();

    let origin_rect = Rect::from_w_h(500.0, BACKGROUND_STEP as f32).top_left_of(window);

    let color_start = convert_to_lsrgb(71, 6, 94);

    let color_end = convert_to_lsrgb(215, 47, 62);

    let gradient = Gradient::new(vec![color_start, color_end, color_start]);

    for i in (0..WINDOW_HEIGHT).step_by(BACKGROUND_STEP) {
        let gradient_step: f32 = i as f32 / WINDOW_HEIGHT as f32;
        let color = gradient.get(gradient_step);

        draw.rect()
            .x(origin_rect.x())
            .y(origin_rect.y() - i as f32)
            .wh(origin_rect.wh())
            .color(color);
    }

    draw.background().color(SKYBLUE);
    // draw.ellipse()
    //     .x_y(-100.0, 200.0)
    //     .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
