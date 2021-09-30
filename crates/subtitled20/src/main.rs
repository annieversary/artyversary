use nannou::prelude::*;
use utils::*;

// based on https://nbickford.wordpress.com/2011/04/03/the-minsky-circle-algorithm/

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    points: Vec<Vec2>,
    base_hue: f32,
}

fn model(_app: &App) -> Model {
    Model {
        points: (0..1000).map(|_| Vec2::ONE * 6.0).collect(),
        base_hue: 0.0,
    }
}

fn advance(mut point: Vec2, t: f32) -> Vec2 {
    let d = map_sin(t * 1.1, 0.01, 0.12);
    let e = (t.sqrt() * 0.01 + 0.3).min(1.1);
    point.x -= (point.y * d).floor();
    point.y += (point.x * e).floor();
    point
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
    model.base_hue += random::<f32>();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    drawing::draw_soft_bg(&draw, app, BLACK, 0.01);

    for &p in &model.points {
        let h = random_range(model.base_hue, model.base_hue + 60.0) / 360.0;
        draw.ellipse()
            .radius(map_sin(p.x.powi(3) + p.y.powi(7) + t * 0.1, 1.0, 3.0))
            .xy(p * 7.0)
            .color(hsla(h.fract(), 1.0, 0.5, 0.1));
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
