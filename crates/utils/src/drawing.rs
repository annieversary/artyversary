use nannou::{color::IntoLinSrgba, draw::properties::ColorScalar, prelude::*};

pub fn draw_soft_bg(draw: &Draw, app: &App, color: impl IntoLinSrgba<ColorScalar>, alpha: f32) {
    if app.elapsed_frames() <= 1 {
        draw.background().color(color);
    } else {
        let mut color = color.into_lin_srgba();
        color.alpha = alpha;

        let win = app.window_rect();
        draw.rect().wh(win.wh()).color(color);
    }
}

/// Draws the opposite of a box
pub fn draw_exterior(draw: &Draw, size: f32, color: impl IntoLinSrgba<ColorScalar> + Clone) {
    draw.quad()
        .points(
            vec2(size, -1000.0),
            vec2(size, 1000.0),
            vec2(1000.0, 1000.0),
            vec2(1000.0, -1000.0),
        )
        .color(color.clone());
    draw.quad()
        .points(
            vec2(-size, -1000.0),
            vec2(-size, 1000.0),
            vec2(-1000.0, 1000.0),
            vec2(-1000.0, -1000.0),
        )
        .color(color.clone());
    draw.quad()
        .points(
            vec2(-1000.0, size),
            vec2(-1000.0, 1000.0),
            vec2(1000.0, 1000.0),
            vec2(1000.0, size),
        )
        .color(color.clone());
    draw.quad()
        .points(
            vec2(-1000.0, -size),
            vec2(-1000.0, -1000.0),
            vec2(1000.0, -1000.0),
            vec2(1000.0, -size),
        )
        .color(color);
}
