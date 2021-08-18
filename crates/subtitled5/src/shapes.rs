use nannou::prelude::*;

pub struct Shapes(Vec<Shape>);
impl Shapes {
    pub fn draw(&self, draw: Draw) {
        for shape in &self.0 {
            match shape {
                Shape::Triangle { center, radius } => {
                    let points = vec![vec2(0., 1.), vec2(0.866, -0.5), vec2(-0.866, -0.5)]
                        .into_iter()
                        .map(|v| v * *radius + *center)
                        .collect::<Vec<_>>();
                    draw.polyline().weight(3.).points_closed(points).color(PINK);
                }
                Shape::Circle { center, radius } => {
                    draw.ellipse()
                        .radius(*radius)
                        .no_fill()
                        .stroke_weight(3.)
                        .xy(*center)
                        .stroke_color(PINK);
                }
                Shape::Line { center, radius } => {}
            }
        }
    }

    pub fn new() -> Self {
        let root = Shape::random(Vec2::ZERO, 100.0);

        let mut vec = vec![];

        for _ in 0..3 {
            let (c, r) = root.get_random_point_distance();
            let s = Shape::random(c, r);

            let children = random_range(0, 3);
            for _ in 0..children {
                let (c, r) = s.get_random_point_distance();
                let s = Shape::random(c, r);
                vec.push(s);
            }

            vec.push(s);
        }
        vec.push(root);

        Self(vec)
    }
}

pub enum Shape {
    Triangle { center: Vec2, radius: f32 },
    Circle { center: Vec2, radius: f32 },
    Line { center: Vec2, radius: f32 },
}
impl Shape {
    fn random(center: Vec2, radius: f32) -> Self {
        match random_range(0, 3) {
            0 => Self::tri(center, radius),
            1 => Self::line(center, radius),
            _ => Self::circ(center, radius),
        }
    }

    fn tri(center: Vec2, radius: f32) -> Self {
        Self::Triangle { center, radius }
    }
    fn circ(center: Vec2, radius: f32) -> Self {
        Self::Circle { center, radius }
    }
    fn line(center: Vec2, radius: f32) -> Self {
        Self::Line { center, radius }
    }

    fn get_random_point_distance(&self) -> (Vec2, f32) {
        match self {
            Shape::Triangle { center, radius } => {
                let points = vec![vec2(0., 1.), vec2(0.866, -0.5), vec2(-0.866, -0.5)]
                    .into_iter()
                    .map(|v| v * *radius + *center)
                    .collect::<Vec<_>>();

                let denom = random_range(1.0, 5.0).floor();
                (points[random_range(0, points.len())], radius / denom)
            }
            Shape::Circle { center, radius } => {
                let point = match random_range(0, 4) {
                    0 => *center + Vec2::X * *radius,
                    1 => *center - Vec2::X * *radius,
                    2 => *center + Vec2::Y * *radius,
                    _ => *center - Vec2::Y * *radius,
                };

                let denom = random_range(1.0, 5.0).floor();
                (point, radius / denom)
            }
            Shape::Line { center, radius } => {
                let point = todo!("one of the two vertices");

                let denom = random_range(1.0, 5.0).floor();
                (point, radius / denom)
            }
        }
    }
}
