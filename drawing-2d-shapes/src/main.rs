use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);

    // quad, tri
    /*
    let p1 = pt2(-10.0, -20.0);
    let p2 = pt2(10.0, -30.0);
    let p3 = pt2(15.0, 40.0);
    let p4 = pt2(-20.0, 35.0);

    draw.quad()
        .color(STEELBLUE)
        .w(300.0)
        .h(200.0)
        .points(p1, p2, p3, p4);
    
    draw.tri()
        .color(SKYBLUE)
        .points(p1, p2, p3);
    */
    
    // line
    /*
    let start_point = pt2(-30.0, -20.0);
    let end_point = pt2(40.0, 40.0);

    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(4.0)
        .color(SPRINGGREEN);
    */
    
    // polyline (sine wave)
    /*
    let points = (0..50).map(|i| {
        let x = i as f32 - 25.0;
        let point = pt2(x, x.sin()) * 20.0;
        (point, STEELBLUE)
    });
    draw.polyline().weight(3.0).points_colored(points);
    */

    // polyline (octagon)
    /*
    let radius = 150.0;
    let points = (0..=360).step_by(45).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * radius;
        let y = radian.cos() * radius;
        (pt2(x, y), STEELBLUE)
    });
    draw.polyline().weight(3.0).points_colored(points);
    */

    // polygon (octagon)
    let radius = 150.0;
    let points = (0..=360).step_by(45).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * radius;
        let y = radian.cos() * radius;
        (pt2(x, y), STEELBLUE)
    });
    draw.polygon().points_colored(points);

    draw.to_frame(app, &frame).unwrap();
}
