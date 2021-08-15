use nannou::prelude::*;

use utils::record::record;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

const fn color(red: u8, green: u8, blue: u8) -> Rgb<u8> {
    Rgb {
        red,
        green,
        blue,
        standard: std::marker::PhantomData::<nannou::color::encoding::Srgb>,
    }
}

const BG: Rgb<u8> = color(0, 5, 5);
const BALL: Rgb<u8> = color(255, 255, 255);

const COLORS: &[Rgb<u8>] = &[
    color(125, 22, 22),
    color(107, 19, 16),
    color(69, 15, 16),
    color(72, 0, 50),
    color(223, 0, 84),
    color(255, 139, 106),
    color(255, 214, 194),
];

const fn sm_color(i: i32, j: i32) -> Rgb<u8> {
    if i == 0 && j == 0 {
        BG
    } else {
        BALL
    }
}
const fn bg_color(i: i32, j: i32) -> Rgb<u8> {
    if i == 0 && j == 0 {
        return BALL;
    }

    let x = i * i * i * j + j * j + 3 + i + j + i * j;
    COLORS[x.abs() as usize % COLORS.len()]
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    let t = frame.nth() as f32 / 60.0;

    draw.background().color(BG);

    for i in -4..=4 {
        for j in -4..=4 {
            let speed = map_range((i + j).abs() as f32, 0.0, 8.0, 0.1, 2.0);
            let dis = (t * speed).sin() * 10.0;

            let circ_offset = vec2(
                (t + (i * i) as f32).sin() * dis,
                (t + (j * i) as f32).cos() * dis,
            );

            let x = i as f32 * 100.0 * (t * 0.7).sin();
            let y = j as f32 * 100.0 * (t * 0.7).cos();

            draw.ellipse()
                .w_h(40.0, 40.0)
                .x_y(x + circ_offset.x, y + circ_offset.y)
                .color(bg_color(i, j));
            draw.ellipse()
                .w_h(10.0, 10.0)
                .x_y(x, y)
                .color(sm_color(i, j));
        }
    }

    draw.to_frame(app, &frame).unwrap();

    record(app, &frame);
}
