use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

const CELL_SIZE: f64 = 5.0;
const GRID_SIZE: usize = 300;

pub struct App {
    gl: GlGraphics,
    board: Vec<Vec<bool>>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const ALIVE: [f32; 4] = [0.0, 0.8, 0.0, 1.0]; // Green
        const DEAD: [f32; 4] = [0.0, 0.0, 0.0, 1.0]; // Black

        self.gl.draw(args.viewport(), |c, gl| {
            clear([1.0; 4], gl);

            for i in 0..GRID_SIZE {
                for j in 0..GRID_SIZE {
                    let x = j as f64 * CELL_SIZE;
                    let y = i as f64 * CELL_SIZE;
                    let square = rectangle::square(0.0, 0.0, CELL_SIZE);
                    let color = if self.board[i][j] { ALIVE } else { DEAD };
                    rectangle(color, square, c.transform.trans(x, y), gl);
                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.board = next_board_state(&self.board);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Game of Life", [1280, 1280])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        board: initialize_board(GRID_SIZE, GRID_SIZE),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

fn initialize_board(rows: usize, cols: usize) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    (0..rows)
        .map(|_| (0..cols).map(|_| rng.gen()).collect())
        .collect()
}

fn next_board_state(board: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_board = board.clone();
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let live_neighbors = count_live_neighbors(i, j, board);
            let cell = board[i][j];
            new_board[i][j] = match (cell, live_neighbors) {
                (true, x) if x < 2 || x > 3 => false,
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => cell,
            };
        }
    }
    new_board
}

fn count_live_neighbors(y: usize, x: usize, board: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    for i in 0..3 {
        for j in 0..3 {
            if !(i == 1 && j == 1) {
                let new_y = (y + i + board.len() - 1) % board.len();
                let new_x = (x + j + board[0].len() - 1) % board[0].len();
                if board[new_y][new_x] {
                    count += 1;
                }
            }
        }
    }
    count
}
