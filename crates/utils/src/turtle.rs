use nannou::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum TurtleAlphabet {
    Line,
    Move,
    Right,
    Left,
    Push,
    Pop,
}

pub struct Turtle {
    position: Vec2,
    pub rotation: f32,
    len: f32,
    theta: f32,
    stack: Vec<(Vec2, f32)>,
}

impl Turtle {
    pub fn new(position: Vec2, len: f32, theta: f32) -> Self {
        Self {
            position,
            rotation: 0.0,
            len,
            theta,
            stack: Default::default(),
        }
    }

    pub fn advance(&mut self, draw: &Draw, l: TurtleAlphabet) {
        match l {
            TurtleAlphabet::Line | TurtleAlphabet::Move => {
                let next = self.position + Vec2::ONE.rotate(self.rotation) * self.len;
                draw.line().points(self.position, next);
                self.position = next;
            }
            // TurtleAlphabet::Move => {
            //     self.position += Vec2::ONE.rotate(self.rotation) * self.len;
            // }
            TurtleAlphabet::Right => self.rotation = (self.rotation - self.theta).rem_euclid(TAU),
            TurtleAlphabet::Left => self.rotation = (self.rotation + self.theta).rem_euclid(TAU),
            TurtleAlphabet::Push => self.stack.push((self.position, self.rotation)),
            TurtleAlphabet::Pop => {
                if let Some((a, b)) = self.stack.pop() {
                    self.position = a;
                    self.rotation = b;
                }
            }
        }
    }

    pub fn advance_many(&mut self, draw: &Draw, l: &[TurtleAlphabet]) {
        for &l in l {
            self.advance(draw, l)
        }
    }
}
