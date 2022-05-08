use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    angle: f32,
    ang_noise: f64,
    radius: f32,
    radius_noise: f64,
    x_noise: f64,
    y_noise: f64,
    perlin: Perlin,
    stroke_col: i32,
    stroke_change: i32,
}

impl Model {
    fn center(&self) -> Vec2 {
        pt2(
            noise(self.perlin, self.x_noise) * 100.0 - 50.0,
            noise(self.perlin, self.y_noise) * 100.0 - 50.0,
        )
    }
}

fn model(_app: &App) -> Model {
    Model {
        angle: 0.0,
        ang_noise: random_range(0.0, 10.0),
        radius: 0.0,
        radius_noise: random_range(0.0, 10.0),
        x_noise: random_range(0.0, 10.0),
        y_noise: random_range(0.0, 10.0),
        perlin: Perlin::new(),
        stroke_col: 254,
        stroke_change: -1,
    }
}

fn update(_app: &App, model: &mut Model, _event: Update) {
    let perlin = Perlin::new();

    model.radius_noise += 0.005;
    model.radius = noise(perlin, model.radius_noise) * 550.0 + 1.0;

    model.ang_noise += 0.005;
    model.angle += noise(perlin, model.ang_noise) * 6.0 - 3.0;
    if model.angle > 360.0 {
        model.angle -= 360.0;
    }
    if model.angle < 0.0 {
        model.angle += 360.0;
    }

    model.x_noise += 0.01;
    model.y_noise += 0.01;

    model.stroke_col += model.stroke_change;
    if model.stroke_col > 254 {
        model.stroke_change = -1;
    }
    if model.stroke_col < 0 {
        model.stroke_change = 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
	if app.elapsed_frames() == 0 {
		draw.background().color(WHITE);
	}

    let center = model.center();

    let rad = deg_to_rad(model.angle);
    let start = pt2(
        center.x + model.radius * rad.cos(),
        center.y + model.radius * rad.sin(),
    );

    let opp_rad = rad + PI;
    let end = pt2(
        center.x + model.radius * opp_rad.cos(),
        center.y + model.radius * opp_rad.sin(),
    );

    let col = map_range(model.stroke_col, 0, 255, 0.0, 1.0);

    draw.line()
        .start(start)
        .end(end)
        .weight(1.0)
        .rgba(col, col, col, 0.6);

    draw.to_frame(app, &frame).unwrap();
}

fn noise(noise: Perlin, x: f64) -> f32 {
    map_range(noise.get([x, 0.0]), -1.0, 1.0, 0.0, 1.0)
}
