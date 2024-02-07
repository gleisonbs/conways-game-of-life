use clearscreen;
use core::time;
use std::i32;
use std::thread;

const BOARD_ROWS: usize = 30;
const BOARD_COLS: usize = 50;

pub struct GameOfLife {
    grid: [[bool; BOARD_COLS]; BOARD_ROWS],
}

impl GameOfLife {
    pub fn new(pattern: Vec<Vec<bool>>) -> GameOfLife {
        let mut game_of_life = GameOfLife {
            grid: [[false; BOARD_COLS]; BOARD_ROWS],
        };

        for (row_index, row) in pattern.iter().enumerate() {
            for (col_index, _) in row.iter().enumerate() {
                game_of_life.grid[row_index][col_index] = pattern[row_index][col_index];
            }
        }

        game_of_life
    }

    pub fn draw_board(&self) {
        thread::sleep(time::Duration::from_millis(1000));
        println!();
        clearscreen::clear().unwrap();
        for i in 0..BOARD_ROWS {
            for j in 0..BOARD_COLS {
                print!("|{}", if self.grid[i][j] { "." } else { " " });
                if j == BOARD_COLS - 1 {
                    print!("|")
                }
            }
            println!();
        }
    }

    fn get_neighbors(&self, row: usize, col: usize) -> i32 {
        let mut count_neighbors = 0;
        if row > 0 {
            count_neighbors += self.grid[row - 1][col] as i32;
            if col > 0 {
                count_neighbors += self.grid[row - 1][col - 1] as i32;
            }
            if col < BOARD_COLS - 1 {
                count_neighbors += self.grid[row - 1][col + 1] as i32;
            }
        }
        if row < BOARD_ROWS - 1 {
            count_neighbors += self.grid[row + 1][col] as i32;
            if col > 0 {
                count_neighbors += self.grid[row + 1][col - 1] as i32;
            }
            if col < BOARD_COLS - 1 {
                count_neighbors += self.grid[row + 1][col + 1] as i32;
            }
        }

        if col > 0 {
            count_neighbors += self.grid[row][col - 1] as i32;
        }
        if col < BOARD_COLS - 1 {
            count_neighbors += self.grid[row][col + 1] as i32;
        }

        count_neighbors
    }

    fn is_cell_alive(&self, neighbors: i32, is_alive: bool) -> bool {
        (neighbors == 2 && is_alive) || (neighbors == 3)
    }

    pub fn next_generation(&mut self) {
        let mut temp_board = GameOfLife {
            grid: [[false; BOARD_COLS]; BOARD_ROWS],
        };

        for row in 0..BOARD_ROWS {
            for col in 0..BOARD_COLS {
                let neighbors = self.get_neighbors(row, col);
                let is_alive = self.grid[row][col];
                temp_board.grid[row][col] = self.is_cell_alive(neighbors, is_alive)
            }
        }

        for row in 0..BOARD_ROWS {
            for col in 0..BOARD_COLS {
                self.grid[row][col] = temp_board.grid[row][col];
            }
        }
    }
}
