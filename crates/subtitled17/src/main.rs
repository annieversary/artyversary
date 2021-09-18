use nannou::prelude::*;
use utils::{curves::*, drawing::draw_soft_bg, *};

const POINTS: usize = 10;
const SIZE: f32 = 300.0;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    curves: Vec<Chaikin>,
}

fn model(_app: &App) -> Model {
    let curves = (0..10)
        .map(|_| {
            Chaikin::new(
                (0..POINTS)
                    .map(|_| vec2_range(-1., 1.) * SIZE)
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    Model { curves }
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    let _t = app.elapsed_frames() as f32 / 60.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    draw_soft_bg(&draw, app, SNOW, 0.01);

    for (i, curve) in model.curves.iter().enumerate() {
        let div = map_sin(t * 0.7 + (2 * i) as f32, 0.15, 0.35);
        let h = map_sin(t * 0.3 + i as f32, 0., 1.);

        draw.polyline().stroke_weight(1.5).points_colored(
            curve
                .points(div, 10)
                .into_iter()
                .enumerate()
                .map(|(i, p)| (p, hsla((h + i as f32 / 10000.0).fract(), 0.5, 0.5, 0.5))),
        );
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
