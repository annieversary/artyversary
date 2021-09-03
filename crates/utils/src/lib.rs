pub mod color;
pub mod drawing;
pub mod record;
pub mod sequences;

use nannou::prelude::*;

/// Maps the sine of v to (out_min, out_max)
pub fn map_sin(v: f32, out_min: f32, out_max: f32) -> f32 {
    map_range(v.sin(), -1.0, 1.0, out_min, out_max)
}
/// Maps the cosine of v to (out_min, out_max)
pub fn map_cos(v: f32, out_min: f32, out_max: f32) -> f32 {
    map_range(v.cos(), -1.0, 1.0, out_min, out_max)
}

pub trait Vec2Extension {
    fn atan2(self) -> f32;
    fn yy(self) -> Self;
    fn yx(self) -> Self;
    fn xx(self) -> Self;
}
impl Vec2Extension for Vec2 {
    fn atan2(self) -> f32 {
        self.x.atan2(self.y)
    }
    fn yy(self) -> Self {
        vec2(self.y, self.y)
    }
    fn yx(self) -> Self {
        vec2(self.y, self.x)
    }
    fn xx(self) -> Self {
        vec2(self.x, self.x)
    }
}
pub trait Tup2Extension {
    fn to_vec2(self) -> Vec2;
}
impl Tup2Extension for (f32, f32) {
    fn to_vec2(self) -> Vec2 {
        self.into()
    }
}

pub fn vec2_range(min: f32, max: f32) -> Vec2 {
    vec2(random_range(min, max), random_range(min, max))
}
