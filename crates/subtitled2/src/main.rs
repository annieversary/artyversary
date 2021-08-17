use nannou::prelude::*;

use utils::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    points: Vec<Vec2>,
}

const PNUM: i32 = 500;

fn model(_app: &App) -> Model {
    let points = (0..PNUM)
        .map(|i| (i as f32 * TAU / PNUM as f32).sin_cos().into())
        .map(|v: Vec2| v * 100.0)
        .collect();
    Model { points }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();

    if frame.nth() == 1 {
        draw.background().color(BLACK);
    } else {
        let win = app.window_rect();
        draw.rect().wh(win.wh()).color(srgba(0., 0., 0., 0.03));
    }

    let big_r = map_sin(t * 2.0, 1.0, 2.0);

    let points: Vec<Vec2> = model
        .points
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, p)| {
            let o = map_sin(t * 10.0 + 10.0 * TAU * (i as f32 / PNUM as f32), 0.8, 1.2);
            p * o * big_r
        })
        .collect();

    for p in points {
        let h = ((p.atan2() - t * 0.2) % TAU) / TAU;
        let s = 0.7;
        let l = map_range(p.length(), 100.0 * 0.8, 200.0 * 1.2, 0.1, 0.5);

        draw.ellipse().color(hsl(h, s, l)).xy(p).w_h(10.0, 10.0);
    }

    draw.to_frame(app, &frame).unwrap();

    utils::record::record(app, &frame);
}
