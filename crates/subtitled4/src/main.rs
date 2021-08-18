use nannou::{color::Mix, prelude::*};

use utils::*;

fn main() {
    nannou::app(model)
        .size(800, 600)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    points: Vec<Point>,
    bg_color: Color,
    fg_color: Color,
}
type Color = rgb::Rgb<nannou::color::encoding::Linear<nannou::color::encoding::Srgb>>;
struct Point {
    pos: Vec2,
}

const GOLDEN: f32 = 0.618033988;

fn model(_app: &App) -> Model {
    Model {
        bg_color: srgb(1.0f32, 250.0 / 255.0, 250.0 / 255.0).into_linear(),
        fg_color: srgb(189.0 / 255.0, 178.0 / 255.0, 1.0).into_linear(),

        points: (0..100)
            .map(|i| {
                let theta = i as f32 * TAU * GOLDEN;
                let r = theta.sqrt();
                let x = r * theta.cos();
                let y = r * theta.sin();
                let pos = vec2(x, y) * 15.0;
                Point { pos }
            })
            .collect(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for p in &mut model.points {
        p.pos += vec2(p.pos.y, -p.pos.x).normalize() * 19.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(model.bg_color);
    }

    for p in &model.points {
        let m = map_sin(t * 0.3 + p.pos.length(), 0.0, 1.0).powi(3);

        let color: Color = model.fg_color.mix(&model.bg_color, m);
        draw.ellipse().xy(p.pos).radius(10.0).color(color);
    }

    draw.to_frame(app, &frame).unwrap();

    utils::record::record(app, &frame);
}
