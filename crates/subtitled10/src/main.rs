use nannou::prelude::*;
use utils::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

// http://paulbourke.net/fractals/clifford/

#[derive(Clone, Copy, Debug)]
struct Params {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}
struct Model {
    points: Vec<Vec2>,
    base_hue: f32,

    /// params for the attractor
    params: Params,

    // modulation values for each param
    a_mul: f32,
    a_add: f32,
    b_mul: f32,
    b_add: f32,
    c_mul: f32,
    c_add: f32,
    d_mul: f32,
    d_add: f32,

    // max range for each param
    // this would be a const, but acos isn't a const fn
    min_a: f32,
    max_a: f32,
    min_b: f32,
    max_b: f32,
    min_c: f32,
    max_c: f32,
    min_d: f32,
    max_d: f32,
}

fn model(_app: &App) -> Model {
    Model {
        params: Params {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
        },

        a_mul: random_range(0.3, 1.0) * random_range(-1.0, 1.0).signum(),
        b_mul: random_range(0.3, 1.0) * random_range(-1.0, 1.0).signum(),
        c_mul: random_range(0.3, 1.0) * random_range(-1.0, 1.0).signum(),
        d_mul: random_range(0.3, 1.0) * random_range(-1.0, 1.0).signum(),

        a_add: random_range(0.3, 2.0),
        b_add: random_range(0.3, 2.0),
        c_add: random_range(0.3, 2.0),
        d_add: random_range(0.3, 2.0),

        // magic numbers from here:
        // http://paulbourke.net/fractals/clifford/paul_richards/main.cpp
        min_a: f32::acos(1.6 / 2.0),
        max_a: f32::acos(1.3 / 2.0),
        min_b: f32::acos(-0.6 / 2.0),
        max_b: f32::acos(1.7 / 2.0),
        min_c: f32::acos(-1.2 / 2.0),
        max_c: f32::acos(0.5 / 2.0),
        min_d: f32::acos(1.6 / 2.0),
        max_d: f32::acos(1.4 / 2.0),

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

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = app.elapsed_frames() as f32 / 60.0;

    // modulate params
    model.params.a = map_sin(t * model.a_mul + model.a_add, model.min_a, model.max_a);
    model.params.b = map_sin(t * model.b_mul + model.b_add, model.min_b, model.max_b);
    model.params.c = map_sin(t * model.c_mul + model.c_add, model.min_c, model.max_c);
    model.params.d = map_sin(t * model.d_mul + model.d_add, model.min_d, model.max_d);

    // advance all points in list
    let mut last = model.points.last().unwrap().clone();
    for v in &mut model.points {
        last = advance(model.params, last);
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
        let h = random_range(model.base_hue, model.base_hue + 120.0) / 360.0;
        draw.ellipse()
            .radius(0.2)
            .xy(150.0 * p)
            .color(hsla(h.fract(), 1.0, 0.5, 0.1));
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
