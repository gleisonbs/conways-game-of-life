mod engine;

use std::io::{stdout, Write};

use nannou::prelude::*;

use engine::game_of_life::GameOfLife;
use engine::patterns::Pattern;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const ROWS: u32 = 30;
const COLS: u32 = 30;

struct Model {
    click_happened: bool,
    mouse_coordinates: Point2,
    rows: u32,
    cols: u32,
}

fn main() {
    const BOARD_ROWS: usize = 30;
    const BOARD_COLS: usize = 30;

    let glider = Pattern::glider();
    let mut game_of_life = GameOfLife::new(BOARD_ROWS, BOARD_COLS, glider);
    // loop {
    game_of_life.draw_board();
    game_of_life.next_generation();
    // }

    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .event(event)
        .view(view)
        .size(WIDTH, HEIGHT)
        .build()
        .unwrap();
    Model {
        click_happened: false,
        mouse_coordinates: pt2(0.0, 0.0),
        rows: ROWS,
        cols: COLS,
    }
}

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

    pt2(new_x_value, new_y_value)
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(pos) => {
            model.mouse_coordinates = pt2(pos.x, pos.y);
        }
        MousePressed(button) => {
            model.click_happened = button == MouseButton::Left;
            translate_click(model);
        }
        _other => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.click_happened {
        println!("Click happened at {}", model.mouse_coordinates);
        model.click_happened = false;
        stdout().flush().unwrap();
    }
    // println!("{}", model.mouse_coordinates);
}

fn view(_app: &App, _model: &Model, _frame: Frame) {}
