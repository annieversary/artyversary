use std::time::Duration;

use nannou::prelude::*;
use utils::{
    lsystems::LSystem,
    turtle::{Turtle, TurtleAlphabet},
};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(600, 520)
        .run();
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
    if frame.nth() == 0 {
        draw.background().color(SNOW);
    }

    if frame.nth() % 60 == 0 {
        let iters = t.trunc() as usize % 10;

        use TurtleAlphabet::*;
        let mut sys = LSystem::new(
            vec![Line, Right, Right, Line, Right, Right, Line],
            Box::new(|i| match i {
                Line => vec![
                    Line, Right, Right, Line, Right, Right, Line, Right, Right, Move,
                ],
                Move => vec![Move, Move],
                a @ _ => vec![a],
            }),
        );
        let h = app.window_rect().h() / 2.0;
        let w = app.window_rect().w() / 2.0;

        let mut turtle = Turtle::new(vec2(-w, -h), 2.85 * w * 0.5.powi(iters as i32), TAU / 3.0);
        turtle.rotation = -PI / 4.0;

        let a = sys.nth(iters);
        turtle.advance_many(&draw, &a);
        // turtle.advance(&draw, TurtleAlphabet::Line);
        // turtle.advance(&draw, TurtleAlphabet::Right);
        // turtle.advance(&draw, TurtleAlphabet::Line);
    }

    // cause otherwise it goes too fast when recording and it can't save the frames in time lmao
    std::thread::sleep(Duration::from_millis(5));

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
