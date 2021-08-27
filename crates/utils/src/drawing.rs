use nannou::{color::IntoLinSrgba, draw::properties::ColorScalar, prelude::*};

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
