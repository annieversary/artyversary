use nannou::{color::Mix, prelude::*};

use utils::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    points: Vec<Vec2>,
}

fn model(_app: &App) -> Model {
    let r = 300.0;

    Model {
        points: (0..100)
            .map(|_| {
                let r = r * random::<f32>().sqrt();
                let theta = random::<f32>() * TAU;
                let x = r * theta.cos();
                let y = r * theta.sin();
                vec2(x, y)
            })
            .collect(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for p in &mut model.points {
        *p += vec2(p.y, -p.x) * 0.01;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    let bg_color = srgb(1.0f32, 250.0 / 255.0, 250.0 / 255.0);
    let fg_color = srgb(189.0 / 255.0, 178.0 / 255.0, 1.0);

    if frame.nth() == 1 {
        draw.background().color(bg_color);
    } else {
        let win = app.window_rect();
        draw.rect()
            .wh(win.wh())
            .color(srgba(1., 250.0 / 255.0, 250.0 / 255.0, 0.001));
    }

    for p in &model.points {
        let m = map_sin(t * 1.2 + p.length() / 100.0, 0.0, 1.0).powi(3);

        let color = fg_color.into_linear().mix(&bg_color.into_linear(), m);
        draw.ellipse().xy(*p).radius(10.0).color(color);
    }

    draw.to_frame(app, &frame).unwrap();
}
