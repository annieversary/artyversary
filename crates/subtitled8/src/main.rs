use nannou::prelude::*;
use utils::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

const POINTS: usize = 10;
const NUM: usize = 200;

struct Model {
    points: Vec<Vec2>,
    offsets: Vec<Vec2>,

    bg: Vec<(Vec2, f32)>,
}
impl Model {
    fn points(&self) -> Vec<Vec2> {
        self.points
            .windows(2)
            .flat_map(|p| {
                let a = p[0];
                let b = p[1];
                (0..NUM).map(move |i| {
                    let i = i as f32 / NUM as f32;
                    (1.0 - i) * a + i * b
                })
            })
            .zip(self.offsets.iter())
            .map(|(a, &b)| a + b)
            .collect()
    }
}

fn model(_app: &App) -> Model {
    let points = (0..POINTS)
        .map(|_| vec2(random_range(-1., 1.), random_range(-1., 1.)) * 200.0)
        .collect::<Vec<_>>();

    let offsets = vec![Vec2::ZERO; NUM * POINTS];

    let bg = sequences::Halton::points(2.0, 3.0)
        .take(10000)
        .map(|v| (v - Vec2::splat(0.5)) * 1100.0)
        .filter(|v| v.x.abs() > 220.0 || v.y.abs() > 220.0)
        .map(|v| (v, 0.5))
        .collect();

    Model {
        points,
        offsets,
        bg,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let t = app.elapsed_frames() as f32 / 60.0;

    for p in &mut model.offsets {
        *p *= 0.95;
        *p += 0.3 * vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0));
    }

    for (i, p) in model.points.iter_mut().enumerate() {
        let a = i as f32 * 0.1;
        p.x = (t * a + i as f32).sin();
        p.y = (t * 1.3 * a + i as f32).cos();
        *p *= 200.0;
    }

    for (i, p) in model.bg.iter_mut().enumerate() {
        p.1 = (t * 2.0 + i as f32).sin() + 1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(SNOW);
    } else {
        // we only want fading on the inner box,
        // outside of it we want a hard reset

        let win = app.window_rect();
        draw.rect()
            .wh(win.wh())
            .color(srgba(1.0, 250.0 / 255.0, 250.0 / 255.0, 0.001));

        draw.quad()
            .points(
                vec2(220.0, -1000.0),
                vec2(220.0, 1000.0),
                vec2(1000.0, 1000.0),
                vec2(1000.0, -1000.0),
            )
            .color(SNOW);
        draw.quad()
            .points(
                vec2(-220.0, -1000.0),
                vec2(-220.0, 1000.0),
                vec2(-1000.0, 1000.0),
                vec2(-1000.0, -1000.0),
            )
            .color(SNOW);
        draw.quad()
            .points(
                vec2(-1000.0, 220.0),
                vec2(-1000.0, 1000.0),
                vec2(1000.0, 1000.0),
                vec2(1000.0, 220.0),
            )
            .color(SNOW);
        draw.quad()
            .points(
                vec2(-1000.0, -220.0),
                vec2(-1000.0, -1000.0),
                vec2(1000.0, -1000.0),
                vec2(1000.0, -220.0),
            )
            .color(SNOW);
    }

    draw.polyline().points(model.points()).color(BLACK);

    for &(p, r) in &model.bg {
        draw.ellipse().xy(p).radius(r).color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
