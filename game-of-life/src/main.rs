use nannou::prelude::*;
use rand::prelude::*;

const COL_N: u32 = 128;                      // フィールドの列数
const ROW_N: u32 = 72;                       // フィールドの行数
const CELL_SIZE: u32 = 8;                    // セル一つあたりの1辺の長さ（ポイント)
const INITIAL_ALIVE_RATIO: f32 = 0.25;       // フィールド生成時に生存しているセルの割合
const ITERATION_INTERVAL_SECONDS: f32 = 0.1; // フィールドの更新間隔（秒）

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    last_iteration_time: f32, // フィールドを更新した最後の時間（プログラム開始からの経過秒数）
    field: Field,
}

// フィールドの状態を管理するための構造体
struct Field {
    col_n: u32,             // フィールドの列数（横方向のセル数）
    row_n: u32,             // フィールドの行数（縦方向のセル数）
    matrix: Vec<Vec<Cell>>, // セルを行列の中に格納する
}

impl Field {
    // フィールドを生成するためのメソッド
    fn new(row_n: u32, col_n: u32, alive_ratio: f32) -> Field {
       let mut matrix: Vec<Vec<Cell>> = vec![]; 
       let mut rng = thread_rng();
       for _x in 0..col_n {
           let mut col: Vec<Cell> = vec![];
           for _y in 0..row_n {
               // alive_ratioの確率で生きているセルを生成する
               let state = if rng.gen::<f32>() < alive_ratio { CellState::Alive } else { CellState::Dead };
               // セルの色は生成時に決定。同じ色が次世代でも受け継がれる
               let cell = Cell::new(state, 0.1, 0.2, rng.gen_range(0.5..1.0));
               col.push(cell);
           }
           matrix.push(col);
       }
       Field{ row_n, col_n, matrix }
    }

    // 現在のフィールドの状態をもとに次の状態へ移行するためのメソッド
    fn iterate(&mut self) {
        // 次の状態を計算
        let new_matrix = self.matrix.iter().enumerate().map(|(x, col)| {
            col.iter().enumerate().map(|(y, cell)| {
                let neighbor_count = self.alive_neighbour_count_from(x, y);
                match cell.state {
                    CellState::Alive => {
                        match neighbor_count {
                            0 | 1 => cell.dead_clone(), // 過疎
                            2 | 3 => *cell, // 生存
                            _ => cell.dead_clone(), // 過密
                        }
                    },
                    CellState::Dead=> {
                        match neighbor_count {
                            3 => cell.alive_clone(), // 誕生
                            _ => *cell,
                        }
                    },
                }
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        self.matrix = new_matrix;
    }

    // フィールドの中から特定の場所のセルを取り出すためのメソッド
    fn get_cell(&self, x: usize, y: usize) -> Cell {
        self.matrix[x][y]
    }

    // 与えられた場所に接しているセルのうち生存しているものの数を返すメソッド
    fn alive_neighbour_count_from(&self, from_x: usize, from_y: usize) -> u32 {
        let mut alive_count = 0u32;
        
        for offset_x in -1..=1 {
            for offset_y in -1..=1 {
                // 自分の位置であればスキップ
                if offset_x == 0 && offset_y == 0 {
                    continue;
                }
                let x = from_x as i32 + offset_x;
                let y = from_y as i32 + offset_y;
                // 領域外であればスキップ
                if x < 0 || (self.col_n as i32) <= x || y < 0 || (self.row_n as i32) <= y {
                    continue;
                }
                // 該当のセルを取得して生存していたらカウントアップ
                let cell = self.get_cell(x as usize, y as usize);
                match cell.state {
                    CellState::Alive => {
                        alive_count += 1;
                    },
                    _ => {},
                }
            }
        }

        alive_count
    }
}

// 個別のセルを表現するための構造体。CloneとCopyを利用できるようにする
#[derive(Clone, Copy)]
struct Cell {
    state: CellState,
    rgb: [f32; 3],
}

impl Cell {
    fn new(state: CellState, r: f32, g: f32, b: f32) -> Cell {
        Cell { state, rgb: [r, g, b] }
    }

    // 同じセルの生きているクローンを返すメソッド
    fn alive_clone(&self) -> Cell {
        let mut clone = self.clone();
        clone.state = CellState::Alive;
        clone
    }

    // 同じセルの死んでいるクローンを返すメソッド
    fn dead_clone(&self) -> Cell {
        let mut clone = self.clone();
        clone.state = CellState::Dead;
        clone
    }
}

#[derive(Clone, Copy)]
enum CellState {
    Alive, // 生存
    Dead,  // 死滅
}

fn model(app: &App) -> Model {
    let window_width = COL_N * CELL_SIZE;
    let window_height = ROW_N * CELL_SIZE;
    app.new_window()
       .size(window_width, window_height)
       .view(view)
       .build()
       .unwrap();

    let model = Model{
        last_iteration_time: 0.0,
        field: Field::new(ROW_N, COL_N, INITIAL_ALIVE_RATIO),
    };

    model
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // 最後のフィールド更新時間から既定のインターバルが経過していれば次の状態に移行する
    if (app.time - model.last_iteration_time) >= ITERATION_INTERVAL_SECONDS {
        model.field.iterate();
        model.last_iteration_time = app.time; // 最後のイテレーションの時間をモデルの中に保持
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.11, 0.12, 0.13);

    // 画面左上を基準に描画していくため、左上のRectを取得
    let win = app.window_rect();
    let top_left_rect = Rect::from_w_h(CELL_SIZE as f32, CELL_SIZE as f32).top_left_of(win);
    
    // セル一つ一つを描画する
    for (x, col) in model.field.matrix.iter().enumerate() {
        for (y, cell) in col.iter().enumerate() {
            match cell.state {
                CellState::Dead => {},
                CellState::Alive => {
                    let shift_x = x as f32 * CELL_SIZE as f32;
                    let shift_y = -1.0 * y as f32 * CELL_SIZE as f32;
                    let current_rect = top_left_rect.shift_x(shift_x).shift_y(shift_y);
                    draw.rect()
                        .xy(current_rect.xy())
                        .wh(current_rect.wh())
                        .rgb(cell.rgb[0], cell.rgb[1], cell.rgb[2]);
                },
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
