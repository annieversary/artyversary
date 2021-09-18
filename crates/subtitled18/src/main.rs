use nannou::prelude::*;
use utils::{drawing::draw_soft_bg, *};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    let _t = app.elapsed_frames() as f32 / 60.0;
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    draw_soft_bg(&draw, app, BLACK, 1.0);

    let count = 10;
    for a in 0..360 {
        for i in 1..=count {
            let a = a as f32;
            let fi = i as f32;

            let r = 1500.0 * (t + 0.01 * a + fi).cos();
            let r = r.powf(0.9 * fi / count as f32);
            // alternate directions of spin for each layer
            let s = if i % 2 == 0 { 1.0 } else { -1.0 } * map_sin(a * fi * fi, 0.1, 1.0);

            let p = r * (s * t + fi + a).sin_cos().to_vec2();

            // jitter
            let p = p + map_sin(t * 0.3 + TAU * 0.75, 0., 7.0) * vec2_circ();
            let sat = map_sin(t * 0.3 + TAU * 0.75, 0.4, 0.9);

            let r = map_range(r, 0., 500.0, 0.5, 13.0).sqrt();

            draw.ellipse()
                .radius(r)
                .xy(p)
                .color(hsl(a / 360.0, sat, 0.5));
        }
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
