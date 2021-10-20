use nannou::prelude::*;
use utils::{drawing::draw_soft_bg, *};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

enum Type {
    Square,
    Circle,
}
struct Shape {
    size: f32,
    r: f32,
    color: f32,
    t: Type,
}

struct Model {
    shapes: Vec<Shape>,
}

fn model(_app: &App) -> Model {
    Model {
        shapes: (0..1000)
            .map(|_| {
                let size = random_range(7.0, 18.0);
                let r = random_range(100.0, 300.0);
                let color = (random_range(0.0, 80.0) / 360.0).fract();
                let t = if random::<f32>() > 0.3 {
                    Type::Circle
                } else {
                    Type::Square
                };
                Shape { size, r, color, t }
            })
            .collect(),
    }
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    let _t = app.elapsed_frames() as f32 / 60.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    draw_soft_bg(&draw, &app, SNOW, 1.03);

    let count = model.shapes.len() as f32;
    for (i, shape) in model.shapes.iter().enumerate() {
        let angle = 360.0 * i as f32 / count;
        let angle = angle + t * map_sin(angle.powi(4), 0.2, 0.7);

        let r = shape.r * map_sin(t + i as f32, 0.5, 1.3);

        let size = shape.size * map_sin(0.004 * t + 2.0 * i as f32, 0.5, 1.5);

        match shape.t {
            Type::Circle => {
                let color = (shape.color + 0.01 * t + angle * 0.0001).fract();
                let color = hsl(color, map_sin(i.pow(3) as f32, 0.4, 0.7), 0.5);

                let p = r * angle.sin_cos().to_vec2();

                draw.ellipse().radius(size).xy(p).color(color);
            }
            Type::Square => {
                let p = r * (-angle).sin_cos().to_vec2();
                let rotation = t + i as f32;

                let points = vec![
                    vec2(1.0, 1.0),
                    vec2(-1.0, 1.0),
                    vec2(-1.0, -1.0),
                    vec2(1.0, -1.0),
                ]
                .into_iter()
                .map(|v| {
                    vec2(
                        v.x * rotation.cos() - v.y * rotation.sin(),
                        v.x * rotation.sin() + v.y * rotation.cos(),
                    )
                })
                .map(|v| v * size + p)
                .collect::<Vec<_>>();

                draw.polyline()
                    .stroke_weight(1.5)
                    .points_closed(points)
                    .color(SNOW);
            }
        };
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
