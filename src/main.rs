use nannou::{color::Gradient, prelude::*};

mod constants;
use constants::CONSTANTS;

mod enums;

mod utils;
use utils::convert_to_lsrgb;

mod sun;
use sun::SunAfterimage;

mod reflection;
use reflection::Reflection;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    sun_afterimages: Vec<SunAfterimage>,
    reflections: Vec<Reflection>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(CONSTANTS.WINDOW_WIDTH, CONSTANTS.WINDOW_HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let mut sun_afterimages = vec![];

    for _ in 0..=CONSTANTS.SUN_AFTERIMAGES_COUNT {
        sun_afterimages.push(SunAfterimage::new());
    }

    let mut reflections = vec![];

    for _ in 0..=CONSTANTS.REFLECTION_LINES_COUNT {
        reflections.push(Reflection::new());
    }

    Model {
        _window,
        sun_afterimages,
        reflections,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for sun_afterimage in &mut _model.sun_afterimages {
        sun_afterimage.update();
    }

    for reflection in &mut _model.reflections {
        reflection.update();
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();

    // background
    let origin_rect = Rect::from_w_h(500.0, CONSTANTS.BACKGROUND_STEP as f32).top_left_of(window);

    let color_start = convert_to_lsrgb(71, 6, 94);
    let color_end = convert_to_lsrgb(215, 47, 62);

    let gradient = Gradient::new(vec![color_start, color_end, color_start]);

    for i in (0..CONSTANTS.WINDOW_HEIGHT).step_by(CONSTANTS.BACKGROUND_STEP) {
        let gradient_step: f32 = i as f32 / CONSTANTS.WINDOW_HEIGHT as f32;
        let color = gradient.get(gradient_step);

        draw.rect()
            .x(origin_rect.x())
            .y(origin_rect.y() - i as f32)
            .wh(origin_rect.wh())
            .color(color);
    }

    // sun afterimages
    for sun in &_model.sun_afterimages {
        draw.ellipse()
            .xy(sun.position)
            .color(Rgba8::new(255, 183, 15, 100));
    }

    // sun main
    draw.ellipse()
        .xy(CONSTANTS.SUN_POSITION)
        .color(Rgb8::new(255, 183, 15));

    // sun reflection
    for reflection in &_model.reflections {
        draw.line()
            .points(
                Point2::new(
                    reflection.position.x - reflection.width / 2 as f32,
                    reflection.position.y,
                ),
                Point2::new(
                    reflection.position.x + reflection.width / 2 as f32,
                    reflection.position.y,
                ),
            )
            .weight(reflection.thickness)
            .color(rgba(255, 255, 255, reflection.alpha));
    }

    draw.to_frame(app, &frame).unwrap();
}
