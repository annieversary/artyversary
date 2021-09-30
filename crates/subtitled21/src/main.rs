use nannou::prelude::*;
// use utils::*;

// from https://fabiensanglard.net/doom_fire_psx/

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

fn shader(i: usize, j: usize, w: usize, h: usize, t: f32) -> Rgb {
    let x = i as f32 / w as f32;
    let y = j as f32 / h as f32;
    let pos = vec2(x, y);

    // srgb(pos.x, pos.y, 0.0)

    // let a = (pos - vec2(0.5, 0.5)).length();
    // srgb(a, a, a)

    creation(pos, t)
}

fn creation(pos: Vec2, t: f32) -> Rgb {
    let mut l = t;
    let mut z = t;
    let mut c = [0.0; 3];

    for i in 0..3 {
        let mut uv = pos;
        let mut p = pos;

        p -= Vec2::splat(0.5);
        z += 0.07;
        l = p.length();
        uv += p / l * (z.sin() + 1.) * abs((l * 9. - z * 2.).sin());
        c[i] = 0.01 / ((vec2(uv.x.fract(), uv.y.fract()) - Vec2::splat(0.5)).abs()).length();
    }

    srgb(c[0] / l, c[1] / l, c[2] / l)
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 120.0;

    let draw = app.draw();
    draw.background().color(BLACK);

    let (l, top, w, h) = app.window_rect().l_t_w_h();

    let cols = (w / BALL_SIZE) as usize;
    let rows = (h / BALL_SIZE) as usize;

    for i in 0..cols {
        for j in 0..rows {
            let x = l + j as f32 * BALL_SIZE;
            let y = top - i as f32 * BALL_SIZE;
            let p = vec2(x, y)
                + (t * 0.4).min(5.0)
                    * 10.0
                    * vec2(
                        (t * 0.7 + (i * j) as f32).sin(),
                        (t * 0.9 + (2 * i * j) as f32).cos(),
                    );

            let color = shader(i, j, cols, rows, t);

            draw.quad()
                .points(
                    p + BALL_SIZE * vec2(1., 1.),
                    p + BALL_SIZE * vec2(1., -1.),
                    p + BALL_SIZE * vec2(-1., -1.),
                    p + BALL_SIZE * vec2(-1., 1.),
                )
                .xy(p)
                .color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
