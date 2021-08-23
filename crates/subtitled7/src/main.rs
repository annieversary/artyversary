use nannou::prelude::*;
use std::collections::VecDeque;
use utils::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    points: VecDeque<(Vec2, (f32, f32))>,
}

const GOLDEN: f32 = 0.618033988;

fn point(i: usize) -> Vec2 {
    let theta = i as f32 * TAU * GOLDEN;
    let r = theta.sqrt();
    let x = r * theta.cos();
    let y = r * theta.sin();
    vec2(x, y)
}

fn color() -> (f32, f32) {
    (
        random_range(345.0, 355.0) / 360.0,
        random_range(75.0, 93.0) / 100.0,
    )
}

fn model(_app: &App) -> Model {
    Model {
        points: (0..2000).map(point).map(|a| (a * 10.0, color())).collect(),
    }
}

fn circ(p: Vec2, center: Vec2, r: f32) -> f32 {
    (p - center).length() - r
}

fn spinny_circles(p: Vec2, t: f32) -> f32 {
    let mut c = vec![(Vec2::ZERO, 1.0)];

    let mut ext = (0..7)
        .map(|i| {
            let v: Vec2 = (t + TAU * i as f32 / 7.0).sin_cos().into();
            (v * 100.0, 10.0)
        })
        .collect::<Vec<_>>();
    c.append(&mut ext);
    let mut ext = (0..7)
        .map(|i| {
            let v: Vec2 = (-t + TAU * i as f32 / 7.0).sin_cos().into();
            (v * 250.0, 10.0)
        })
        .collect::<Vec<_>>();
    c.append(&mut ext);

    c.iter()
        .fold(f32::MAX, |acc, &(c, r)| acc.min(circ(p, c, r)))
}

fn segment(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = (pa.dot(ba) / ba.length_squared()).clamp(0.0, 1.0);
    return (pa - ba * h).length();
}
fn pentagram(p: Vec2, center: Vec2, t: f32) -> f32 {
    let points = (0..5)
        .map(|i| {
            let v: Vec2 = (-t + TAU * i as f32 / 5.0).sin_cos().into();
            v * 100.0 - center
        })
        .collect::<Vec<_>>();

    (0..5)
        .map(|i| (points[i], points[(i + 2) % 5]))
        .fold(f32::MAX, |acc, (a, b)| acc.min(segment(p, a, b)))
}
fn pentagrams(p: Vec2, t: f32) -> f32 {
    (0..5)
        .map(|i| {
            let v: Vec2 = (t / 10.0 + TAU * i as f32 / 5.0).sin_cos().into();
            let v = v * 330.0;
            pentagram(p, v, t)
        })
        .fold(f32::MAX, |acc, a| acc.min(a))
}

fn f(p: Vec2, t: f32) -> f32 {
    spinny_circles(p, t).min(pentagrams(p, t))
}

fn norm_f(p: Vec2, t: f32) -> Vec2 {
    const H: f32 = 0.0001;
    let k = vec2(1.0, -1.0);
    return (k * f(p + k * H, t)
        + k.yy() * f(p + k.yy() * H, t)
        + k.yx() * f(p + k.yx() * H, t)
        + k.xx() * f(p + k.xx() * H, t))
    .normalize();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = app.elapsed_frames() as f32 / 120.0;
    for (p, _) in &mut model.points {
        *p -= norm_f(*p, t) * f(*p, t).signum();
    }

    if app.elapsed_frames() > 20 {
        for _ in 0..10 {
            model
                .points
                .push_back((6.0 * point(random_range(0, 1000)), color()));
        }
    }

    // remove a bunch of them every once in a while
    // since we're already not running real time, who cares if we pause for a bit rigth?
    if app.elapsed_frames() % 200 == 0 {
        // drain_filter but i'm too lazy for nightly
        let mut i = 0;
        while i < model.points.len() {
            if f(model.points[i].0, t) < 0.0 {
                model.points.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _t = frame.nth() as f32 / 120.0;

    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BLACK);
    } else {
        let win = app.window_rect();
        draw.rect().wh(win.wh()).color(srgba(0., 0.0, 0.0, 0.005));
    }

    for &(p, (h, l)) in &model.points {
        draw.ellipse().color(hsl(h, 1.0, l)).xy(p).radius(1.0);
    }

    draw.to_frame(app, &frame).unwrap();

    utils::record::record(app, &frame);
}
