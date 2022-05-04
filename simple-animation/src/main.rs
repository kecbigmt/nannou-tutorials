// 参考記事
// https://zenn.dev/pvcresin/articles/4b9edacc87527a

use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(600, 400).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    // アプリケーションが起動してからの秒数をtに格納
    let t = app.time;

    // sin. cosを使って円運動を表現
    let center = pt2(t.cos(), t.sin()) * 100.0;

    draw.rect()
        .x_y(center.x, center.y)
        .w_h(100.0, 100.0)
        .color(BLUE);

    draw.to_frame(app, &frame).unwrap();
}
