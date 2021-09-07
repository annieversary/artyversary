use nannou::prelude::*;
// use utils::*;

// from https://fabiensanglard.net/doom_fire_psx/

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    fire: Vec<Vec<f32>>,
    colors: Vec<Rgb8>,
    uwu: wgpu::Texture,
}

const WIDTH: usize = 400;
const HEIGHT: usize = 300;
const BALL_SIZE: f32 = 3.0;

fn model(app: &App) -> Model {
    let fire = vec![vec![0.0; WIDTH]; HEIGHT];

    let colors = vec![
        (0x07, 0x07, 0x07),
        (0x1F, 0x07, 0x07),
        (0x2F, 0x0F, 0x07),
        (0x47, 0x0F, 0x07),
        (0x57, 0x17, 0x07),
        (0x67, 0x1F, 0x07),
        (0x77, 0x1F, 0x07),
        (0x8F, 0x27, 0x07),
        (0x9F, 0x2F, 0x07),
        (0xAF, 0x3F, 0x07),
        (0xBF, 0x47, 0x07),
        (0xC7, 0x47, 0x07),
        (0xDF, 0x4F, 0x07),
        (0xDF, 0x57, 0x07),
        (0xDF, 0x57, 0x07),
        (0xD7, 0x5F, 0x07),
        (0xD7, 0x5F, 0x07),
        (0xD7, 0x67, 0x0F),
        (0xCF, 0x6F, 0x0F),
        (0xCF, 0x77, 0x0F),
        (0xCF, 0x7F, 0x0F),
        (0xCF, 0x87, 0x17),
        (0xC7, 0x87, 0x17),
        (0xC7, 0x8F, 0x17),
        (0xC7, 0x97, 0x1F),
        (0xBF, 0x9F, 0x1F),
        (0xBF, 0x9F, 0x1F),
        (0xBF, 0xA7, 0x27),
        (0xBF, 0xA7, 0x27),
        (0xBF, 0xAF, 0x2F),
        (0xB7, 0xAF, 0x2F),
        (0xB7, 0xB7, 0x2F),
        (0xB7, 0xB7, 0x37),
        (0xCF, 0xCF, 0x6F),
        (0xDF, 0xDF, 0x9F),
        (0xEF, 0xEF, 0xC7),
        (0xFF, 0xFF, 0xFF),
    ]
    .into_iter()
    .map(|c| c.into())
    .collect();

    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("blahaj.png");
    let uwu = wgpu::Texture::from_path(app, img_path).unwrap();

    let mut model = Model { fire, colors, uwu };

    // presimulate fire
    for _ in 0..400 {
        advance(app, &mut model);
    }

    model
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let _t = app.elapsed_frames() as f32 / 60.0;

    advance(app, model);
}

fn advance(app: &App, model: &mut Model) {
    let (w, h) = app.window_rect().w_h();

    let cols = (w / BALL_SIZE) as usize;
    let rows = (h / BALL_SIZE) as usize - 50;

    if app.elapsed_frames() > 200 {
        for i in 0..cols {
            model.fire[rows][i] = 0.0;
        }
    } else {
        for i in 0..cols {
            model.fire[rows][i] = 36.0;
        }
    }

    for x in 0..cols {
        for y in 0..rows {
            let disp = random_range(0, 2);
            model.fire[y][(x + disp) % cols] = model.fire[y + 1][x] - random_range(0., 1.0);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let _t = frame.nth() as f32 / 60.0;

    let draw = app.draw();
    draw.background().color(BLACK);

    draw.texture(&model.uwu)
        .w_h(400.0, 282.6)
        .y((-200.0 + frame.nth() as f32).min(0.0));

    let (l, top, w, h) = app.window_rect().l_t_w_h();

    let cols = w / BALL_SIZE;
    let rows = h / BALL_SIZE;

    for (i, row) in model.fire.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let col_index = c.max(0.0).trunc() as usize % model.colors.len();

            if i as f32 <= rows && j as f32 <= cols && col_index != 0 {
                let x = l + j as f32 * BALL_SIZE;
                let y = top - i as f32 * BALL_SIZE;
                let p = vec2(x, y);
                let r = BALL_SIZE;
                // * map_range(i as f32, 0 as f32, rows, 0., 1.0)
                // * random_range(0.9, 1.1);

                draw.quad()
                    .points(
                        p + r * vec2(1., 1.),
                        p + r * vec2(1., -1.),
                        p + r * vec2(-1., -1.),
                        p + r * vec2(-1., 1.),
                    )
                    .xy(p)
                    .color(model.colors[col_index]);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
    utils::record::record(app, &frame);
}
