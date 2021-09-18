use nannou::prelude::*;

pub struct Chaikin {
    pub points: Vec<Vec2>,
}
impl Chaikin {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }

    pub fn points(&self, div: f32, n: usize) -> Vec<Vec2> {
        let mut points = self.points.clone();
        for _ in 0..n {
            let first = *points.first().unwrap();
            let last = *points.last().unwrap();
            points = points
                .windows(2)
                .flat_map(|p| {
                    let a = p[0];
                    let b = p[1];

                    [a + div * (b - a), a + (1.0 - div) * (b - a)]
                })
                .collect();

            points.insert(0, first);
            points.push(last);
        }
        points
    }
}
