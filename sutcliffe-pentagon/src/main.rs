use nannou::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

const STRUT_FACTOR: f32 = 0.2;
const MAX_LEVEL: u8 = 5;

fn main() {
    nannou::app(model)
        .simple_window(view)
        .size(1000, 1000)
        .run();
}

fn model(_app: &App) -> Model {
    Model {
        pentagon: FractalRoot::new(),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    model.pentagon.draw_shape(&draw);

    // Mouse position text.
    let mouse = app.mouse.position();
    let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
    draw.text(&pos)
        .xy(mouse + vec2(0.0, 20.0))
        .font_size(14)
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    pentagon: FractalRoot,
}

struct FractalRoot {
    point_arr: Vec<PointObj>,
    root_branch: Branch,
}

impl FractalRoot {
    fn new() -> FractalRoot {
        let mut point_arr = Vec::with_capacity(5);
        for angle in (0..360).step_by(72) {
            let x = 400.0 * deg_to_rad(angle as f32).cos();
            let y = 400.0 * deg_to_rad(angle as f32).sin();
            point_arr.push(PointObj::new(x, y));
        }
        let root_branch = Branch::new(1, 1, point_arr.clone());
        FractalRoot {
            point_arr,
            root_branch,
        }
    }

    fn draw_shape(&self, draw: &Draw) {
        self.root_branch.draw_me(draw);
    }
}

struct Branch {
    level: u8,
    num: u8,
    outer_points: Vec<PointObj>,
    mid_points: Vec<PointObj>,
    strut_points: Vec<PointObj>,
    childlen: Vec<Rc<RefCell<Branch>>>,
}

impl Branch {
    fn new(level: u8, num: u8, outer_points: Vec<PointObj>) -> Branch {
        let mut branch = Branch {
            level,
            num,
            outer_points,
            mid_points: vec![],
            strut_points: vec![],
            childlen: vec![],
        };
        if level <= MAX_LEVEL {
            branch.mid_points = branch.calc_mid_points();
            branch.strut_points = branch.calc_strut_points();
            branch.childlen.push(Rc::new(RefCell::new(Branch::new(
                level + 1,
                num + 1,
                branch.strut_points.clone(),
            ))));

            for (i, op) in branch.outer_points.iter().enumerate() {
                let j = if (i + 4) < branch.mid_points.len() { i + 4 } else { i - 1 };
                let outer_points = vec![
                    *op,
                    branch.mid_points[i],
                    branch.strut_points[i],
                    branch.strut_points[j],
                    branch.mid_points[j],
                ];
                branch.childlen.push(Rc::new(RefCell::new(Branch::new(
                    level + 1,
                    num + 1,
                    outer_points,
                ))));
            }
        }
        branch
    }

    fn draw_me(&self, draw: &Draw) {
        for (i, p) in self.outer_points.iter().enumerate() {
            let next_i = if i + 1 == self.outer_points.len() {
                0
            } else {
                i + 1
            };
            let next_p = self.outer_points[next_i];
            draw.line()
                .start(pt2(p.x, p.y))
                .end(pt2(next_p.x, next_p.y))
                .weight(5.0 / self.level as f32)
                .color(STEELBLUE);
        }
        for child in &self.childlen {
            for (mp, sp) in self.mid_points.iter().zip(self.strut_points.iter()) {
                draw.ellipse()
                    .x(mp.x)
                    .y(mp.y)
                    .w(15.0 / self.level as f32)
                    .h(15.0 / self.level as f32)
                    .stroke_weight(0.5)
                    .stroke_color(STEELBLUE)
                    .rgba(1.0, 1.0, 1.0, 0.6);
                /*
                draw.ellipse()
                    .x(sp.x)
                    .y(sp.y)
                    .w(15.0)
                    .h(15.0)
                    .stroke_weight(0.5)
                    .stroke_color(STEELBLUE)
                    .rgba(1.0, 1.0, 1.0, 0.6);
                */
                draw.line()
                    .start(pt2(mp.x, mp.y))
                    .end(pt2(sp.x, sp.y))
                    .weight(0.5)
                    .color(STEELBLUE);
            }
            child.borrow().draw_me(draw);
        }
    }

    fn calc_mid_points(&self) -> Vec<PointObj> {
        self.outer_points
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let next_i = if i + 1 == self.outer_points.len() {
                    0
                } else {
                    i + 1
                };
                Branch::calc_mid_point(p, &self.outer_points[next_i])
            })
            .collect::<Vec<_>>()
    }

    fn calc_mid_point(end1: &PointObj, end2: &PointObj) -> PointObj {
        let mx = if end1.x > end2.x {
            end2.x + (end1.x - end2.x) / 2.0
        } else {
            end1.x + (end2.x - end1.x) / 2.0
        };
        let my = if end1.y > end2.y {
            end2.y + (end1.y - end2.y) / 2.0
        } else {
            end1.y + (end2.y - end1.y) / 2.0
        };
        PointObj::new(mx, my)
    }

    fn calc_strut_points(&self) -> Vec<PointObj> {
        self.mid_points
            .iter()
            .enumerate()
            .map(|(i, mp)| {
                // 中点の真向かいにある五角形の頂点のインデックスを計算する
                let opp_i = if (i + 3) < self.mid_points.len() {
                    i + 3
                } else {
                    i + 3 - self.mid_points.len()
                };
                // 中点から真向かいの頂点に垂線を伸ばしたときの途中の点を計算する
                Branch::calc_proj_point(mp, &self.outer_points[opp_i])
            })
            .collect::<Vec<_>>()
    }

    fn calc_proj_point(mp: &PointObj, op: &PointObj) -> PointObj {
        let opp = if op.x > mp.x {
            op.x - mp.x
        } else {
            mp.x - op.x
        };
        let adj = if op.y > mp.y {
            op.y - mp.y
        } else {
            mp.y - op.y
        };
        let px = if op.x > mp.x {
            mp.x + opp * STRUT_FACTOR
        } else {
            mp.x - opp * STRUT_FACTOR
        };
        let py = if op.y > mp.y {
            mp.y + adj * STRUT_FACTOR
        } else {
            mp.y - adj * STRUT_FACTOR
        };

        PointObj::new(px, py)
    }
}

#[derive(Copy, Clone)]
struct PointObj {
    x: f32,
    y: f32,
}

impl PointObj {
    fn new(ex: f32, why: f32) -> PointObj {
        PointObj { x: ex, y: why }
    }
}
