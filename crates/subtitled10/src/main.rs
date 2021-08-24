use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Clone, Copy, Debug)]
struct Params {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Params {
    fn new() -> Self {
        // magic numbers from here:
        // http://paulbourke.net/fractals/clifford/paul_richards/main.cpp
        let min_a = f32::acos(1.6 / 2.0);
        let max_a = f32::acos(1.3 / 2.0);
        let min_b = f32::acos(-0.6 / 2.0);
        let max_b = f32::acos(1.7 / 2.0);
        let min_c = f32::acos(-1.2 / 2.0);
        let max_c = f32::acos(0.5 / 2.0);
        let min_d = f32::acos(1.6 / 2.0);
        let max_d = f32::acos(1.4 / 2.0);

        Self {
            a: random_range(min_a, max_a),
            b: random_range(min_b, max_b),
            c: random_range(min_c, max_c),
            d: random_range(min_d, max_d),
        }
    }
}
struct Model {
    params: Params,
    points: Vec<Vec2>,
    base_hue: f32,
}

fn model(_app: &App) -> Model {
    let params = Params::new();
    dbg!(params);
    Model {
        params,
        base_hue: random_range(0., 360.0),
        points: vec![Vec2::ZERO; 10000],
    }
}

fn advance(params: Params, point: Vec2) -> Vec2 {
    // xn+1 = sin(a yn) + c cos(a xn)
    // yn+1 = sin(b xn) + d cos(b yn)
    vec2(
        (params.a * point.y).sin() + (params.a * point.x).cos() * params.c,
        (params.b * point.x).sin() + (params.b * point.y).cos() * params.d,
    )
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut last = model.points.last().unwrap().clone();

    for v in &mut model.points {
        last = advance(model.params, last);
        *v = last;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _t = frame.nth() as f32 / 60.0;

    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(BLACK);
    }

    for &p in &model.points {
        let h = random_range(model.base_hue, model.base_hue + 60.0) / 360.0;
        draw.ellipse()
            .radius(0.1)
            .xy(150.0 * p)
            .color(hsla(h.fract(), 1.0, 0.5, 0.1));
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
