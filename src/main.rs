mod engine;

use std::io::{stdout, Write};
use std::time::Instant;

use nannou::prelude::*;

use engine::game_of_life::GameOfLife;
use engine::patterns::Pattern;

const UPDATE_DELAY: u128 = 150;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const ROWS: u32 = 40;
const COLS: u32 = 40;

struct Model {
    mouse_coordinates: Point2,
    game_of_life: GameOfLife,
    is_paused: bool,
    tick_time: Instant,
}

fn main() {
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
        mouse_coordinates: pt2(0.0, 0.0),
        game_of_life: GameOfLife::new(ROWS as usize, COLS as usize, Pattern::none()),
        is_paused: true,
        tick_time: Instant::now(),
    }
}

fn draw_ui_grid(draw: &Draw, model: &Model) {
    let square_width = WIDTH as f32 / COLS as f32;
    let square_height = HEIGHT as f32 / ROWS as f32;
    let x_begin = -(WIDTH as f32 / 2.0);
    let y_begin = HEIGHT as f32 / 2.0;

    for row in 0..ROWS {
        for col in 0..COLS {
            let mut color = WHITE;
            if model.game_of_life.grid[row as usize][col as usize] {
                color = YELLOW;
            }

            draw.rect()
                .stroke(RED)
                .stroke_weight(1.0)
                .color(color)
                .w(square_width)
                .h(square_height)
                .x_y(
                    x_begin + (col as f32 * square_width) + square_width / 2.0,
                    y_begin - (row as f32 * square_height) - square_height / 2.0,
                );
        }
    }
}

fn translate_range(value: f32, old_min: f32, old_max: f32, new_min: f32, new_max: f32) -> f32 {
    let old_range = old_max - old_min;
    let new_range = new_max - new_min;
    (((value - old_min) * new_range) / old_range) + new_min
}

fn translate_mouse_click_to_grid_cell(model: &Model) -> Point2 {
    let min_x = WIDTH as f32 / 2.0 - WIDTH as f32;
    let max_x = (WIDTH - WIDTH / 2) as f32;
    let cell_x = translate_range(model.mouse_coordinates.x, min_x, max_x, 0.0, COLS as f32);

    let min_y = (HEIGHT - HEIGHT / 2) as f32;
    let max_y = HEIGHT as f32 / 2.0 - HEIGHT as f32;
    let cell_y = translate_range(model.mouse_coordinates.y, min_y, max_y, 0.0, ROWS as f32);

    pt2(cell_x.floor(), cell_y.floor())
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(pos) => {
            model.mouse_coordinates = pt2(pos.x, pos.y);
        }
        MousePressed(_button) => {
            let box_ticked = translate_mouse_click_to_grid_cell(model);
            model.game_of_life.grid[box_ticked.y as usize][box_ticked.x as usize] =
                !model.game_of_life.grid[box_ticked.y as usize][box_ticked.x as usize];
            stdout().flush().unwrap();
        }
        KeyPressed(key) => {
            if key == Key::Space {
                model.is_paused = !model.is_paused;
            }
        }
        _other => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let now = Instant::now();
    let elapsed = now.duration_since(model.tick_time);
    if !model.is_paused && elapsed.as_millis() > UPDATE_DELAY {
        model.game_of_life.next_generation();
        model.tick_time = Instant::now();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw_ui_grid(&draw, model);

    draw.to_frame(app, &frame).unwrap();
}
