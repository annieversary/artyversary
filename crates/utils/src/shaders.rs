use nannou::prelude::*;

pub fn run<S, T>(app: &App, frame: Frame, ball_size: f32, shader: S, tile_movement: T)
where
    S: Fn(usize, usize, usize, usize, f32) -> Hsl,
    T: Fn(f32, f32) -> Vec2,
{
    let t = frame.nth() as f32 / 120.0;

    let draw = app.draw();
    draw.background().color(BLACK);

    let (l, top, w, h) = app.window_rect().l_t_w_h();

    let cols = (w / ball_size) as usize;
    let rows = (h / ball_size) as usize;

    for i in 0..cols {
        for j in 0..rows {
            let x = l + j as f32 * ball_size;
            let y = top - i as f32 * ball_size;
            let p = tile_movement(x, y);

            let color = shader(i, j, cols, rows, t);

            draw.quad()
                .points(
                    p + ball_size * vec2(1., 1.),
                    p + ball_size * vec2(1., -1.),
                    p + ball_size * vec2(-1., -1.),
                    p + ball_size * vec2(-1., 1.),
                )
                .xy(p)
                .color(color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    super::record::record(app, &frame);
}
