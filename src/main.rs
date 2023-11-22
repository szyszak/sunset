use nannou::{color::Gradient, prelude::*};
use rand::Rng;

mod constants;
use constants::CONSTANTS;

mod enums;
use enums::Direction;

mod utils;
use utils::{calculate_distance, convert_to_lsrgb};

mod sun;
use sun::SunAfterimage;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    sun_afterimages: Vec<SunAfterimage>,
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

    Model {
        _window,
        sun_afterimages,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    for sun_afterimage in &mut _model.sun_afterimages {
        let distance = calculate_distance(
            Vec2::new(sun_afterimage.x, sun_afterimage.y),
            CONSTANTS.SUN_XY,
        );

        match sun_afterimage.direction {
            Direction::Forwards => {
                sun_afterimage.x += sun_afterimage.velocity.x;
                sun_afterimage.y += sun_afterimage.velocity.y;

                if distance >= CONSTANTS.SUN_AFTERIMAGE_MAX_DISTANCE {
                    sun_afterimage.direction = Direction::Backwards;
                }
            }
            Direction::Backwards => {
                sun_afterimage.x -= sun_afterimage.velocity.x;
                sun_afterimage.y -= sun_afterimage.velocity.y;

                if distance <= 0.0 {
                    sun_afterimage.x = CONSTANTS.SUN_XY[0];
                    sun_afterimage.y = CONSTANTS.SUN_XY[1];

                    sun_afterimage.direction = Direction::Forwards;

                    let mut rng = rand::thread_rng();

                    let random_x_vel: f32 = rng.gen_range(
                        -CONSTANTS.SUN_AFTERIMAGE_MAX_VALOCITY
                            ..=CONSTANTS.SUN_AFTERIMAGE_MAX_VALOCITY,
                    );
                    let random_y_vel: f32 = rng.gen_range(
                        -CONSTANTS.SUN_AFTERIMAGE_MAX_VALOCITY
                            ..=CONSTANTS.SUN_AFTERIMAGE_MAX_VALOCITY,
                    );

                    sun_afterimage.velocity = Vec2::new(random_x_vel, random_y_vel);
                }
            }
        }
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
            .xy(Vec2::new(sun.x, sun.y))
            .color(Rgba8::new(255, 183, 15, 100));
    }

    // sun main
    draw.ellipse()
        .xy(Vec2::new(CONSTANTS.SUN_XY[0], CONSTANTS.SUN_XY[1]))
        .color(Rgb8::new(255, 183, 15));

    draw.to_frame(app, &frame).unwrap();
}
