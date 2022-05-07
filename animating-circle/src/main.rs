use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let sin = app.time.sin();
    let slowersin = (app.time / 2.0).sin();

    let boundary = app.window_rect();
    let x = map_range(sin, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersin, -1.0, 1.0, boundary.bottom(), boundary.top());

    draw.ellipse().color(STEELBLUE).x_y(x, y);
    draw.to_frame(app, &frame).unwrap();
}

