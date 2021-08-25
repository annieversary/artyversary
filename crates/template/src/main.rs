use nannou::prelude::*;
use utils::*;

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
    let _t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    draw.background().color(PLUM);

    draw.ellipse().color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
