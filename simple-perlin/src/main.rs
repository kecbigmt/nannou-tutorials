use nannou::prelude::*;
use nannou::noise::*;

const STEP: usize = 10;
const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 100;
const OFFSET: u32 = 20;

fn main() {
    nannou::sketch(view).size(WINDOW_WIDTH, WINDOW_HEIGHT).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE); 

    let win = app.window_rect();
    let boundary_left = win.left() + OFFSET as f32;
    let boundary_right = win.right() - OFFSET as f32;
    
    draw.line()
        .start(pt2(boundary_left, 0.0))
        .end(pt2(boundary_right, 0.0))
        .weight(4.0)
        .color(LIGHTGREY);
    
    let points = {
        let mut noise = random_range(0.0, 10.0);

        let mut points = Vec::new();
        for x in ((boundary_left as i32)..=(boundary_right as i32)).step_by(STEP) {
            let perlin = Perlin::new();
            let y = perlin.get([noise, 0.0]) * 45.0;

            points.push((pt2(x as f32, y as f32), DARKGREY));
            noise = noise + 0.1;
        }

        points
    };
    draw.polyline().weight(4.0).points_colored(points);

    draw.to_frame(app, &frame).unwrap();
}