mod engine;

use std::io::{stdout, Write};
use std::time::Instant;

use nannou::prelude::*;

use engine::game_of_life::GameOfLife;
use engine::patterns::Pattern;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const ROWS: u32 = 30;
const COLS: u32 = 30;

struct Model {
    mouse_coordinates: Point2,
    rows: u32,
    cols: u32,
    game_of_life: GameOfLife,
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .event(event)
        .view(view)
        .size(WIDTH, HEIGHT)
        .build()
        .unwrap();
    Model {
        mouse_coordinates: pt2(0.0, 0.0),
        rows: ROWS,
        cols: COLS,
        game_of_life: GameOfLife::new(ROWS as usize, COLS as usize, Pattern::none()),
    }
}

fn draw_ui_grid() {}

fn translate_click(model: &Model) -> Point2 {
    let min_x = WIDTH as f32 / 2.0 - WIDTH as f32;
    let max_x = (WIDTH - WIDTH / 2) as f32;
    let old_x_range = max_x - min_x;
    let new_x_range = model.rows as f32;
    let new_x_value = ((model.mouse_coordinates.x - min_x) * new_x_range) / old_x_range;

    let min_y = (HEIGHT - HEIGHT / 2) as f32;
    let max_y = HEIGHT as f32 / 2.0 - HEIGHT as f32;
    let old_y_range = max_y - min_y;
    let new_y_range = model.cols as f32;
    let new_y_value = ((model.mouse_coordinates.y - min_y) * new_y_range) / old_y_range;

    pt2(new_x_value.floor(), new_y_value.floor())
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(pos) => {
            model.mouse_coordinates = pt2(pos.x, pos.y);
        }
        MousePressed(_button) => {
            let box_ticked = translate_click(model);
            model.game_of_life.grid[box_ticked.y as usize][box_ticked.x as usize] =
                !model.game_of_life.grid[box_ticked.y as usize][box_ticked.x as usize];
            println!("Click happened at {}", box_ticked);
            stdout().flush().unwrap();
        }
        KeyPressed(key) => {
            if key == Key::Space {
                model.game_of_life.next_generation();
            }
        }
        _other => {}
    }
}

fn view(_app: &App, model: &Model, _frame: Frame) {
    model.game_of_life.draw_board();
}
