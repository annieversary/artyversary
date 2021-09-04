use nannou::prelude::*;
use utils::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    walkers: Vec<Vec2>,
    clustered: Vec<Vec2>,
}

fn model(_app: &App) -> Model {
    Model {
        walkers: (0..5000).map(|_| vec2_range(-1.0, 1.0) * 300.0).collect(),
        clustered: vec![Vec2::ZERO],
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let _t = app.elapsed_frames() as f32 / 60.0;

    // sort them by x to be a bit more efficient with the collision detection
    model
        .clustered
        .sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    let clusters = model.clustered.clone();

    let mut to_remove = vec![];
    for (i, p) in &mut model.walkers.iter_mut().enumerate() {
        *p += vec2_range(-1.0, 1.0) * 3.5 + 0.5 * vec2(p.y, -p.x).normalize();

        const RAD: f32 = 6.0;
        for &c in &clusters {
            // skip the ones outside the x range
            if c.x < p.x - RAD {
                continue;
            }
            if c.x > p.x + RAD {
                break;
            }

            // if it's colliding, make it be a clustered point
            if p.distance_squared(c) < RAD * RAD {
                model.clustered.push(*p);
                to_remove.push(i);
                break;
            }
        }
    }

    // remove the ones that have become clustered
    // backwards cause otherwise you shift the indexes
    for i in to_remove.into_iter().rev() {
        model.walkers.remove(i);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();

    drawing::draw_soft_bg(&draw, app, SNOW, 1.0);

    for &p in &model.clustered {
        let v = map_sin(t + (p.x.powi(2) + p.y.powi(2) + t) * 0.00003, -20.0, 20.0);
        let h = (100.0 + v) / 360.0;
        draw.ellipse().xy(p).radius(4.0).color(hsl(h, 0.5, 0.5));
    }

    for &p in &model.walkers {
        draw.ellipse().xy(p).radius(0.0).color(BLACK);
    }
    // use nannou::rand::seq::SliceRandom;
    // let mut rng = nannou::rand::thread_rng();
    // let n = (frame.nth() as f32 * 0.7) as usize;
    // for _ in 0..n {
    //     let a = model.clustered.choose(&mut rng);
    //     let b = model.clustered.choose(&mut rng);
    //     if let (Some(&a), Some(&b)) = (a, b) {
    //         let v = map_sin(t * 2.0 + a.x * a.y, -20.0, 20.0);
    //         let h = (100.0 + v) / 360.0;

    //         draw.line()
    //             .stroke_weight(3.0)
    //             .points(a, b)
    //             .color(hsla(h, 0.5, 0.5, 0.04));
    //     }
    // }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
