// 参考記事
// https://zenn.dev/pvcresin/articles/4b9edacc87527a

use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(600, 400).run();
}

fn view(app: &App, frame: Frame) {
    // キャンバスを取得
    let draw = app.draw();

    // 背景色を設定
    draw.background().color(WHITE);

    // 1辺100の正方形を原点に表示
    draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(BLUE);

    // フレームに書き出し
    draw.to_frame(app, &frame).unwrap();
}
