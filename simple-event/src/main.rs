// 参考記事
// https://zenn.dev/pvcresin/articles/4b9edacc87527a

use nannou::prelude::*;

fn main() {
    nannou::app(init).run();
}

// モデルの中で、現在の色の状態を保持
struct Model {
    color: Rgb8,
}

fn init(app: &App) -> Model {
    app.new_window()
        .size(600, 400)
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    Model {
        color: gen_random_color(),
    }
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.color = gen_random_color();
}

fn gen_random_color() -> Rgb8 {
    let r = random::<u8>();
    let g = random::<u8>();
    let b = random::<u8>();
    rgb8(r, g, b)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.rect()
        .color(model.color)
        .x_y(0.0, 0.0)
        .w_h(100.0, 100.0);

    draw.to_frame(app, &frame).unwrap();
}

