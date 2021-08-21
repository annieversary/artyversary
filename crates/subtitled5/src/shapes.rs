use nannou::prelude::*;

pub struct Shapes(Vec<Shape>);
impl Shapes {
    pub fn draw(&self, draw: Draw) {
        let w = 1.0;

        for shape in &self.0 {
            match shape {
                &Shape::Triangle {
                    center,
                    radius,
                    rotation,
                } => {
                    let points = vec![vec2(0., 1.), vec2(0.866, -0.5), vec2(-0.866, -0.5)]
                        .into_iter()
                        .map(|v| {
                            vec2(
                                v.x * rotation.cos() - v.y * rotation.sin(),
                                v.x * rotation.sin() + v.y * rotation.cos(),
                            )
                        })
                        .map(|v| v * radius + center)
                        .collect::<Vec<_>>();
                    draw.polyline().weight(w).points_closed(points).color(PINK);
                }
                Shape::Circle { center, radius, .. } => {
                    draw.ellipse()
                        .radius(*radius)
                        .no_fill()
                        .stroke_weight(w)
                        .xy(*center)
                        .stroke_color(PINK);
                }
                &Shape::Line {
                    center,
                    radius,
                    rotation,
                } => {
                    let points = vec![vec2(-1.0, 0.), vec2(1.0, 0.0)]
                        .into_iter()
                        .map(|v| {
                            vec2(
                                v.x * rotation.cos() - v.y * rotation.sin(),
                                v.x * rotation.sin() + v.y * rotation.cos(),
                            )
                        })
                        .map(|v| v * radius + center)
                        .collect::<Vec<_>>();

                    draw.line()
                        .stroke_weight(w)
                        .start(points[0])
                        .end(points[1])
                        .color(PINK);
                }
                Shape::LongLine { center, radius, .. } => {
                    draw.line()
                        .stroke_weight(w)
                        .start(Vec2::ZERO)
                        .end(*center + center.normalize() * *radius)
                        .color(PINK);
                }
                &Shape::Square {
                    center,
                    radius,
                    rotation,
                } => {
                    let points = vec![
                        vec2(1.0, 1.0),
                        vec2(-1.0, 1.0),
                        vec2(-1.0, -1.0),
                        vec2(1.0, -1.0),
                    ]
                    .into_iter()
                    .map(|v| {
                        vec2(
                            v.x * rotation.cos() - v.y * rotation.sin(),
                            v.x * rotation.sin() + v.y * rotation.cos(),
                        )
                    })
                    .map(|v| v * radius + center)
                    .collect::<Vec<_>>();

                    draw.polyline()
                        .stroke_weight(w)
                        .points_closed(points)
                        .color(PINK);
                }
            }
        }
    }

    pub fn new(children: usize) -> Self {
        let root = Shape::random(Vec2::ZERO, 100.0);

        let mut vec = vec![];

        for _ in 0..children {
            let (c, r) = root.get_random_point_distance();
            let s = Shape::random(c, r);

            let children = random_range(0, 5.min(children + 2));
            for _ in 0..children {
                let (c, r) = s.get_random_point_distance();
                let s = Shape::random(c, r);

                let children = random_range(0, 3.min(children + 1));
                for _ in 0..children {
                    let (c, r) = s.get_random_point_distance();
                    let s = Shape::random(c, r);
                    vec.push(s);
                }

                vec.push(s);
            }

            vec.push(s);
        }
        vec.push(root);

        Self(vec)
    }
}

macro_rules! sh {
	( $($id:ident),* ) => {
        pub enum Shape {
            $(
                $id { center: Vec2, radius: f32, rotation: f32 },
            )*
        }
	};
}

sh!(Triangle, Circle, Line, LongLine, Square);

fn random_sq_rot() -> f32 {
    [0.0, PI / 4.0][random_range(0, 2)]
}
fn random_tri_rot() -> f32 {
    [0.0, PI / 3.0][random_range(0, 2)]
}
fn random_line_rot() -> f32 {
    [0.0, PI / 3.0, PI / 2.0, PI / 4.0, PI][random_range(0, 4)]
}

impl Shape {
    fn random(center: Vec2, radius: f32) -> Self {
        match random_range(0, 5) {
            0 => Self::tri(center, radius, random_tri_rot()),
            1 => Self::line(center, radius, random_line_rot()),
            2 if center.distance(Vec2::ZERO) > 0.01 => Self::long_line(center, radius, 0.0),
            3 => Self::square(center, radius, random_sq_rot()),
            _ => Self::circ(center, radius, 0.0),
        }
    }

    fn tri(center: Vec2, radius: f32, rotation: f32) -> Self {
        Self::Triangle {
            center,
            radius,
            rotation,
        }
    }
    fn circ(center: Vec2, radius: f32, rotation: f32) -> Self {
        Self::Circle {
            center,
            radius,
            rotation,
        }
    }
    fn line(center: Vec2, radius: f32, rotation: f32) -> Self {
        Self::Line {
            center,
            radius,
            rotation,
        }
    }
    fn long_line(center: Vec2, radius: f32, rotation: f32) -> Self {
        Self::LongLine {
            center,
            radius,
            rotation,
        }
    }
    fn square(center: Vec2, radius: f32, rotation: f32) -> Self {
        Self::Square {
            center,
            radius,
            rotation,
        }
    }

    fn get_random_point_distance(&self) -> (Vec2, f32) {
        match self {
            &Shape::Triangle {
                center,
                radius,
                rotation,
            } => {
                let points = vec![vec2(0., 1.), vec2(0.866, -0.5), vec2(-0.866, -0.5)]
                    .into_iter()
                    .map(|v| {
                        vec2(
                            v.x * rotation.cos() - v.y * rotation.sin(),
                            v.x * rotation.sin() + v.y * rotation.cos(),
                        )
                    })
                    .map(|v| v * radius + center)
                    .collect::<Vec<_>>();

                let denom = random_range(1.0, 4.0).floor();
                (points[random_range(0, points.len())], radius / denom)
            }
            &Shape::Circle { center, radius, .. } => {
                let point = match random_range(0, 4) {
                    0 => center + Vec2::X * radius,
                    1 => center - Vec2::X * radius,
                    2 => center + Vec2::Y * radius,
                    _ => center - Vec2::Y * radius,
                };

                let denom = random_range(1.0, 4.0).floor();
                (point, radius / denom)
            }
            &Shape::Line { center, radius, .. } => {
                let point = match random_range(0, 2) {
                    0 => center - center.normalize() * radius,
                    _ => center + center.normalize() * radius,
                };

                let denom = random_range(1.0, 4.0).floor();
                (point, radius / denom)
            }
            &Shape::LongLine { center, radius, .. } => {
                let denom = random_range(1.0, 4.0).floor();
                (center + center.normalize() * radius, radius / denom)
            }
            &Shape::Square {
                center,
                radius,
                rotation,
            } => {
                let points = vec![
                    vec2(1.0, 1.0),
                    vec2(-1.0, 1.0),
                    vec2(-1.0, -1.0),
                    vec2(1.0, -1.0),
                ]
                .into_iter()
                .map(|v| {
                    vec2(
                        v.x * rotation.cos() - v.y * rotation.sin(),
                        v.x * rotation.sin() + v.y * rotation.cos(),
                    )
                })
                .map(|v| v * radius + center)
                .collect::<Vec<_>>();

                let denom = random_range(1.0, 4.0).floor();
                (points[random_range(0, 4)], radius / denom)
            }
        }
    }
}
