use nannou::prelude::*;

const NUM_CHILDREN: usize = 4;
const MAX_LEVEL: u32 = 7;

fn main() {
    nannou::app(model).update(update).simple_window(view).size(750, 500).run();
}

struct Model {
    origin: Branch,
}

struct Branch {
    level: u32,
    x: f32,
    y: f32,
    end_x: f32,
    end_y: f32,
    stroke_weight: f32,
    alpha: f32,
    len: f32,
    len_change: f32,
    rot: f32,
    rot_change: f32,
    children: Vec<Branch>,
}

impl Branch {
    fn new(level: u32, ex: f32, why: f32) -> Branch {
        let mut branch = Branch {
            level,
            x: 0.0,
            y: 0.0,
            end_x: 0.0,
            end_y: 0.0,
            stroke_weight: (1.0 / level as f32) * 10.0,
            alpha: 1.0 / level as f32,
            len: (1.0 / level as f32) * random_range(0.0, 500.0),
            rot: random_range(0.0, 360.0),
            len_change: random_range(0.0, 10.0) - 5.0,
            rot_change: random_range(0.0, 10.0) - 5.0,
            children: vec![],
        };

        branch.update_me(ex, why);

        if level < MAX_LEVEL {
            for _ in 0..NUM_CHILDREN {
                branch
                    .children
                    .push(Branch::new(level + 1, branch.end_x, branch.end_y));
            }
        }
        branch
    }

    fn update_me(&mut self, ex: f32, why: f32) {
        self.x = ex;
        self.y = why;

        self.rot += self.rot_change;
        if self.rot > 360.0 {
            self.rot = 0.0;
        } else if self.rot < 0.0 {
            self.rot = 360.0;
        }

        self.len -= self.len_change;
        if self.len < 0.0 {
            self.len_change *= -1.0;
        } else if self.len > 500.0 {
            self.len_change *= -1.0;
        }

        let rad = deg_to_rad(self.rot);
        self.end_x = self.x + (self.len * rad.cos());
        self.end_y = self.y + (self.len * rad.sin());

        for child in self.children.iter_mut() {
            child.update_me(self.end_x, self.end_y);
        }
    }

    fn draw_me(&self, draw: &Draw) {
        if self.level > 1 {
            draw.line()
                .start(pt2(self.x, self.y))
                .end(pt2(self.end_x, self.end_y))
                .weight(self.stroke_weight)
                .rgba(0.0, 0.0, 0.0, self.alpha);

            draw.ellipse()
                .x_y(self.end_x, self.end_y)
                .w(self.len / 12.0)
                .h(self.len / 12.0)
                .stroke_weight(self.stroke_weight)
                .stroke_color(rgba(0.0, 0.0, 0.0, self.alpha))
                .rgba(1.0, 1.0, 1.0, self.alpha);
        }
        for child in self.children.iter() {
            child.draw_me(draw);
        }
    }
}

fn model(_app: &App) -> Model {
    Model {
        origin: Branch::new(1, 0.0, 0.0),
    }
}

fn update(_app: &App, model: &mut Model, _event: Update) {
    model.origin.update_me(1.0, 1.0);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    model.origin.draw_me(&draw);

    draw.to_frame(app, &frame).unwrap();
}
