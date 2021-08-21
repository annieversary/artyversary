use nannou::prelude::*;

mod shapes;
use shapes::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    shape: Shapes,
    count: usize,
}

fn model(_app: &App) -> Model {
    Model {
        shape: Shapes::new(0),
        count: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.elapsed_frames() % 60 == 0 && app.elapsed_frames() > 1 {
        if model.count >= 20 {
            std::process::exit(0);
        }

        model.shape = Shapes::new(random_range(1, 7));

        model.count += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BLACK);
    } else if frame.nth() % 60 > 30 {
        let win = app.window_rect();
        draw.rect().wh(win.wh()).color(srgba(0.0, 0.0, 0.0, 0.2));
    }

    if frame.nth() % 60 == 0 {
        model.shape.draw(draw.clone());
    }

    draw.to_frame(app, &frame).unwrap();

    utils::record::record(app, &frame);
}
