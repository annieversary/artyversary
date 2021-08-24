use nannou::prelude::*;

pub struct Halton {
    i: usize,
    base: f32,
}

impl Halton {
    pub fn new(base: f32) -> Self {
        Self { i: 0, base }
    }

    pub fn points(base1: f32, base2: f32) -> impl Iterator<Item = Vec2> {
        Self::new(base1)
            .zip(Self::new(base2))
            .map(crate::Tup2Extension::to_vec2)
    }
}

impl Iterator for Halton {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut f = 1.0;
        let mut r = 0.0;
        let mut index = self.i as f32;

        while index > 0.0 {
            f /= self.base;
            r += f * (index % self.base);
            index = (index / self.base).floor();
        }
        self.i += 1;
        Some(r)
    }
}
