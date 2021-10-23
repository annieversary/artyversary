use nannou::prelude::*;
use utils::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    points: Vec<Vec2>,
    base_hue: f32,
}

fn model(_app: &App) -> Model {
    Model {
        base_hue: random_range(0., 360.0),
        points: vec![Vec2::splat(0.1); 10000],
    }
}

fn advance(point: Vec2, t: f32) -> Vec2 {
    // pick a random point
    let count = ((t * 0.125).trunc() as usize % 5) + 3;
    let mut points = (0..count)
        .map(|i| (TAU * i as f32 / count as f32).sin_cos().to_vec2())
        .collect::<Vec<_>>();
    points.push(Vec2::ZERO);

    let random = points[random_range(0, count)];

    (random + point) * map_cos(t * PI * 0.25, 0.1, 1.0).powf(1.5)
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = app.elapsed_frames() as f32 / 60.0;

    // advance all points in list
    let mut last = model.points.last().unwrap().clone();
    for v in &mut model.points {
        last = advance(last, t);
        *v = last;
    }

    // move base_hue
    model.base_hue += 0.5;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _t = frame.nth() as f32 / 60.0;

    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(BLACK);
    } else {
        let win = app.window_rect();
        draw.rect().wh(win.wh()).color(srgba(0., 0.0, 0.0, 0.02));
    }

    for &p in &model.points {
        let h = random_range(model.base_hue, model.base_hue + 60.0) / 360.0;
        draw.ellipse()
            .radius(0.2)
            .xy(150.0 * p)
            .color(hsla(h.fract(), 1.0, 0.5, 0.1));
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
