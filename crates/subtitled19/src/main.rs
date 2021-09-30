use nannou::prelude::*;
use utils::{drawing::draw_soft_bg, *};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Walker {
    pos: Vec2,
    vel: Vec2,
    color: f32,
}

struct Model {
    walkers: Vec<Walker>,
}

fn model(_app: &App) -> Model {
    Model {
        walkers: (0..100)
            .map(|_| Walker {
                pos: Vec2::ZERO,
                vel: Vec2::ZERO,
                color: random_range(0.0, 1.0),
            })
            .collect(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let _t = app.elapsed_frames() as f32 / 60.0;

    for walker in &mut model.walkers {
        walker.vel += vec2_range(-2.0, 1.0);
        walker.vel = walker.vel.normalize();

        walker.pos += walker.vel;

        // wrap around the box
        const SIZE: f32 = 100.0;
        if walker.pos.x.abs() > SIZE {
            walker.pos.x = -walker.pos.x;
        }
        if walker.pos.y.abs() > SIZE {
            walker.pos.y = -walker.pos.y;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    draw_soft_bg(&draw, app, SNOW, 0.01);

    for walker in &model.walkers {
        let h = (t * 10.0 + walker.color * 50.0) / 360.0;
        draw.ellipse()
            .xy(walker.pos)
            .radius(1.0)
            .color(hsl(h.fract(), 0.7, 0.5));
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
