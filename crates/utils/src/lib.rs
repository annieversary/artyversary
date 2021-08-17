pub mod color;
pub mod record;

use nannou::prelude::*;

/// Maps the sine of v to (out_min, out_max)
pub fn map_sin(v: f32, out_min: f32, out_max: f32) -> f32 {
    map_range(v.sin(), -1.0, 1.0, out_min, out_max)
}

pub trait Vec2Extension {
    fn atan2(self) -> f32;
}
impl Vec2Extension for Vec2 {
    fn atan2(self) -> f32 {
        self.x.atan2(self.y)
    }
}
