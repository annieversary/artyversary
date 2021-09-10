use std::time::Duration;

use nannou::prelude::*;

// https://sighack.com/post/flood-fill-art-using-random-walks

const R: f32 = 2.0;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Clone)]
struct Point {
    pos: IVec2,
    color: f32,
}

impl Point {
    fn new(pos: IVec2, color: f32) -> Self {
        Self { pos, color }
    }
}

fn neighbors(point: IVec2) -> Vec<IVec2> {
    let mut cube = vec![];
    let p = [-1, 0, 1];
    for &x in &p {
        for &y in &p {
            if (x, y) != (0, 0) {
                cube.push(ivec2(x, y) + point);
            }
        }
    }
    cube
}

struct Model {
    points: Vec<Point>,
    visits: Vec<Point>,
}

fn model(_app: &App) -> Model {
    Model {
        visits: vec![Point::new(IVec2::ZERO, random_range(0.0, 360.0))],
        points: vec![],
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for _ in 0..50 {
        run(app, model)
    }
}
fn run(app: &App, model: &mut Model) {
    let _t = app.elapsed_frames() as f32 / 60.0;

    // take a random out of the visits list
    let point = model.visits.remove(random_range(0, model.visits.len()));
    // visit it and all it's neighbors
    let neighbors = neighbors(point.pos)
        .into_iter()
        .filter(|p| {
            model.visits.iter().find(|point| point.pos == *p).is_none()
                && model.points.iter().find(|point| point.pos == *p).is_none()
        })
        .collect::<Vec<_>>();
    for n in neighbors {
        model
            .visits
            // NOTE: this is where the randomization of the color happens
            .push(Point::new(n, point.color + random_range(-10.0, 10.0)))
    }
    // move it to visited points
    model.points.push(point);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    draw.background().color(SNOW);

    for walker in &model.visits {
        let h = walker.color / 360.0;
        draw.ellipse()
            .xy(walker.pos.as_f32() * R * 2.0)
            .radius(R)
            .color(hsl(h.fract(), 0.5, 0.7));
    }
    for walker in &model.points {
        let h = walker.color / 360.0;
        let p = walker.pos.as_f32() * R;
        draw.quad()
            .points(
                p + R * vec2(1., 1.),
                p + R * vec2(1., -1.),
                p + R * vec2(-1., -1.),
                p + R * vec2(-1., 1.),
            )
            .xy(p)
            .color(hsl(h.fract(), 0.5, 0.5));
    }

    if frame.nth() < 50 {
        // cause otherwise it goes too fast when recording and it can't save the frames in time lmao
        std::thread::sleep(Duration::from_millis(5));
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
