use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    automata: Automata,
    row: usize,
    cube: Vec<Vec4>,
}

fn model(_app: &App) -> Model {
    // generate all the points in the cube
    let mut cube = vec![];
    let p = [-1.0, 1.0];
    for &x in &p {
        for &y in &p {
            for &z in &p {
                for &w in &p {
                    cube.push(vec4(x, y, z, w));
                }
            }
        }
    }

    Model {
        automata: Automata::new(),
        row: 0,
        cube,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let _t = app.elapsed_frames() as f32 / 60.0;

    if app.elapsed_frames() % 10 == 0 {
        model.automata.advance();
        model.row += 1;
    }
}

fn rot(angle: f32) -> Mat4 {
    // 4d rotations https://math.stackexchange.com/questions/1402362/rotation-in-4d
    let m = mat4(
        vec4(angle.cos(), angle.sin(), 0.0, 0.0),
        vec4(-angle.sin(), angle.cos(), 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    ) * mat4(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, angle.cos(), angle.sin(), 0.0),
        vec4(0.0, -angle.sin(), angle.cos(), 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    );
    let angle = -angle * 2.0;
    m * mat4(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, angle.cos(), angle.sin()),
        vec4(0.0, 0.0, -angle.sin(), angle.cos()),
    )
}

fn view(app: &App, model: &Model, frame: Frame) {
    let t = frame.nth() as f32 / 60.0;

    // background

    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(SNOW);
    } else {
        let win = app.window_rect();
        draw.rect()
            .wh(win.wh())
            .color(srgba(1.0, 250.0 / 255.0, 250.0 / 255.0, 0.01));
    }

    // draw automata

    let (l, top, w, h) = app.window_rect().l_t_w_h();

    const BALL_SIZE: f32 = 6.0;
    let cols = w / BALL_SIZE;
    let rows = h / BALL_SIZE;

    for (i, &b) in model.automata.points.iter().enumerate() {
        if b && i as f32 <= cols {
            let x = l + (i as f32 % cols) * BALL_SIZE;
            let y = top - (model.row as f32 % rows) * BALL_SIZE;
            let p = vec2(x, y);
            let h = model.row as f32 / 360.0;
            draw.ellipse()
                .radius(BALL_SIZE / 2.0)
                .xy(p)
                .color(hsl(h.fract(), 0.5, 0.5));
        }
    }

    // draw center square
    draw.rect().w_h(400.0, 400.0).color(SNOW);

    // draw 4d cube
    let p: Vec<_> = model
        .cube
        .iter()
        .map(|&v| {
            // rotate
            let v = rot(t) * v;
            // project onto xz plane cause it's the coolest one
            50.0 * vec2(v.x + v.y, v.z + v.w)
        })
        .collect::<Vec<_>>();

    for (i, (&a, &b)) in p
        .iter()
        .enumerate()
        // make all pairs of points
        .flat_map(|(i, a)| p[i + 1..].iter().map(move |b| (a, b)))
        .enumerate()
    {
        if i % 2 != 0 {
            draw.line().points(a, b).color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}

struct Automata {
    points: Vec<bool>,
    rule: Vec<bool>,
}

impl Automata {
    fn new() -> Self {
        let points = (0..200).map(|_| random::<bool>()).collect();
        let rule = (0..8).map(|_| random::<bool>()).collect();

        Self { points, rule }
    }

    fn advance(&mut self) {
        for i in 0..self.points.len() {
            let i1 = (i + 1) % self.points.len();
            let im1 = if i == 0 { self.points.len() - 1 } else { i - 1 };
            let v = (self.points[im1], self.points[i], self.points[i1]);

            self.points[i] = match v {
                (false, false, false) => self.rule[0],
                (false, false, true) => self.rule[1],
                (false, true, false) => self.rule[2],
                (false, true, true) => self.rule[3],
                (true, false, false) => self.rule[4],
                (true, false, true) => self.rule[5],
                (true, true, false) => self.rule[6],
                (true, true, true) => self.rule[7],
            }
        }
    }
}
