use nannou::prelude::*;
use num::complex::*;
use utils::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(800, 800)
        .run();
}

struct Model {}

const BALL_SIZE: f32 = 3.0;

fn model(_app: &App) -> Model {
    Model {}
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    let _t = app.elapsed_frames() as f32 / 60.0;
}

fn shader(i: usize, j: usize, w: usize, h: usize, t: f32) -> Hsl {
    let x = (i as f32 / w as f32) - 0.5;
    let y = (j as f32 / h as f32) - 0.5;
    let x = x * 7.0;
    let y = y * 7.0;
    let mut v = Complex::new(y, x);

    let a = map_sin(t * 0.5, 0.0, TAU);
    let c = Complex::from_polar(0.7885, a);

    let iters = map_sin(t * 0.1, 10.0, 100.0);

    for i in 0..(iters as usize) {
        v = v.powi(2) + c;

        if v.norm() > 10.0 {
            return hsl(i as f32 / iters, 0.5, 0.5);
        }
    }

    hsl(0.0, 0.0, 0.0)
}

fn view(app: &App, _model: &Model, frame: Frame) {
    utils::shaders::run(app, frame, BALL_SIZE, shader, vec2)
}
