use nannou::prelude::*;
use utils::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(SNOW);
    } else {
        // we only want fading on the inner box,
        // outside of it we want a hard reset

        let win = app.window_rect();
        draw.rect()
            .wh(win.wh())
            .color(srgba(1.0, 250.0 / 255.0, 250.0 / 255.0, 0.01));
    }

    // okay so this is a bit of a hack
    // we draw the ones inside and outside the box in two separate passes
    // on the first one, we draw the ones inside the box, with a box of size 220
    // then we draw the blank stuff outside of a 200 box
    // then we draw the points outside the 200 box

    // this is cause otherwise either the colored lines go outside the box,
    // or the colored lines finish too early
    // this works and it's still real time so
    // don't judge me uwu

    // inner draw
    for i in 0..50 {
        let r = 1.5;
        let r2 = r * (50 + i) as f32 / 50.0;
        let v = vec2((t * r + i as f32).sin(), (t * r2 + (2 * i) as f32).sin()) * 300.0;

        // while inside the box, draw a line to the last point
        if v.x.abs() < 220.0 && v.y.abs() < 220.0 {
            let t = t - 1.0 / 60.0;
            let v_prev = vec2((t * r + i as f32).sin(), (t * r2 + (2 * i) as f32).sin()) * 300.0;

            let h_prev = map_sin((v_prev.x + v_prev.y) / 400.0 + i as f32, 0.0, 1.0);

            draw.line()
                .stroke_weight(4.0)
                .points(v, v_prev)
                .color(hsl(h_prev, 0.5, 0.5));

            let h = map_sin((v.x + v.y) / 400.0 + i as f32, 0.0, 1.0);
            draw.ellipse().radius(2.0).xy(v).color(hsl(h, 0.5, 0.5));
        }
    }

    // draw exteriors
    draw_exterior(&draw);

    // outer draw
    for i in 0..50 {
        let r = 1.5;
        let r2 = r * (50 + i) as f32 / 50.0;
        let v = vec2((t * r + i as f32).sin(), (t * r2 + (2 * i) as f32).sin()) * 300.0;

        if v.x.abs() > 201.0 || v.y.abs() > 201.0 {
            draw.ellipse().radius(2.0).xy(v).color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}

fn draw_exterior(draw: &Draw) {
    draw.quad()
        .points(
            vec2(200.0, -1000.0),
            vec2(200.0, 1000.0),
            vec2(1000.0, 1000.0),
            vec2(1000.0, -1000.0),
        )
        .color(SNOW);
    draw.quad()
        .points(
            vec2(-200.0, -1000.0),
            vec2(-200.0, 1000.0),
            vec2(-1000.0, 1000.0),
            vec2(-1000.0, -1000.0),
        )
        .color(SNOW);
    draw.quad()
        .points(
            vec2(-1000.0, 200.0),
            vec2(-1000.0, 1000.0),
            vec2(1000.0, 1000.0),
            vec2(1000.0, 200.0),
        )
        .color(SNOW);
    draw.quad()
        .points(
            vec2(-1000.0, -200.0),
            vec2(-1000.0, -1000.0),
            vec2(1000.0, -1000.0),
            vec2(1000.0, -200.0),
        )
        .color(SNOW);
}
