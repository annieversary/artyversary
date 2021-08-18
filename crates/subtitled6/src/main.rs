use nannou::prelude::*;
use utils::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

const D: f32 = 300.0;

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    draw.background().color(SNOW);

    let div = map_cos(t, 2.0, 50.0).floor() as i32;
    let offset = D / div as f32;
    let resolution = map_cos(t, 5.0, 50.0).floor() * 2.0;

    for i in (0..=div).rev() {
        let fi = i as f32;
        draw.scissor(Rect::from_corners(vec2(-D, 0.0), vec2(D, D)))
            .ellipse()
            .resolution(resolution)
            .w_h(offset * fi, offset * fi)
            .x(offset * (div as f32 - fi) / 2.0)
            .color(srgb(
                i as f32 / div as f32,
                0.0,
                (div - i + 1) as f32 / div as f32,
            ));
    }
    for i in (0..=div).rev() {
        let fi = i as f32;
        draw.scissor(Rect::from_corners(vec2(D, 0.0), vec2(-D, -D)))
            .ellipse()
            .resolution(resolution)
            .w_h(offset * fi, offset * fi)
            .x(-offset * (div as f32 - fi) / 2.0)
            .color(srgb(
                (div - i + 1) as f32 / div as f32,
                0.0,
                i as f32 / div as f32,
            ));
    }

    draw.to_frame(app, &frame).unwrap();

    utils::record::record(app, &frame);
}
