use nannou::prelude::*;
use nannou::rand::prelude::*;
use rand_distr::Normal;
use utils::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    lines: Vec<Vec<Vec2>>,
}

fn lines(max_width: f32) -> Vec<Vec<Vec2>> {
    let normal = Normal::new(0.0, 0.2).unwrap();

    (0..20)
        .map(|_| {
            let mut line = vec![Vec2::Y];

            // generate the y coordinates for all points
            let n = random_range(4, 12);
            let mut ys = (0..n).map(|_| random_range(-1.0, 1.0)).collect::<Vec<_>>();
            ys.sort_by(|a, b| b.partial_cmp(a).unwrap());

            // make the actual points and add them to the vec
            for y in ys {
                let w = map_range(1.0 - y.abs(), 0.0, 1.0, 0.01, max_width);
                let x = nannou::rand::thread_rng().sample::<f32, _>(normal) * w;
                line.push(vec2(x, y));
            }

            line.push(-Vec2::Y);

            line
        })
        .collect()
}

fn model(_app: &App) -> Model {
    Model { lines: lines(0.0) }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = app.elapsed_frames() as f32 / 60.0;

    if app.elapsed_frames() % 2 == 0 {
        model.lines = lines(map_sin(t, 0.1, 1.0));
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    draw.background().color(BLACK);

    for line in &model.lines {
        draw.polyline()
            .stroke_weight(1.0)
            .points(line.iter().map(|v| *v * 300.0))
            .color(PINK);
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
