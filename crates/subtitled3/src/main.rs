use nannou::prelude::*;

use utils::color::color;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

const COLORS: &[Rgb<u8>] = &[
    color(125, 22, 22),
    color(107, 19, 16),
    color(69, 15, 16),
    color(72, 0, 50),
    color(223, 0, 84),
    color(255, 139, 106),
    color(255, 214, 194),
];
fn rand_col() -> Rgb<u8> {
    COLORS[random_range(0, COLORS.len())]
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

const BG: Rgb<u8> = color(250, 235, 215);

fn view(app: &App, _model: &Model, frame: Frame) {
    let _t = frame.nth() as f32 / 60.0;

    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(BG);
    } else {
        let win = app.window_rect();
        draw.rect()
            .wh(win.wh())
            .color(srgba(1., 250.0 / 255.0, 250.0 / 255.0, 0.01));
    }

    let win = app.window_rect();
    let xs = win.x.start;
    let xe = win.x.end;
    let ys = win.y.start;
    let ye = win.y.end;

    let points: Vec<_> = (0..200)
        .map(|_| vec2(random_range(xs, xe), random_range(ys, ye)))
        .collect();

    for p in points {
        draw.ellipse().xy(p).radius(10.0).color(rand_col());
    }

    macro_rules! big_circ {
        ($draw:ident, $axis:ident, $from:expr) => {
            let mut r = $from;
            while r.abs() > 1. {
                $draw.ellipse().$axis(r).radius(r / 2.0).color(BG);
                r /= 2.0;
            }
        };
    }
    big_circ!(draw, x, 800.0);
    big_circ!(draw, x, -800.0);
    big_circ!(draw, y, 800.0);
    big_circ!(draw, y, -800.0);

    draw.to_frame(app, &frame).unwrap();

    utils::record::record(app, &frame);
}
