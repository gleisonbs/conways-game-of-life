use clearscreen;
use core::time;
use std::i32;
use std::thread;

pub struct GameOfLife {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(rows: usize, cols: usize, pattern: Vec<Vec<bool>>) -> GameOfLife {
        let mut grid = Vec::new();

        for (row_index, row) in pattern.iter().enumerate() {
            let mut vec_row = Vec::new();
            for (col_index, _) in row.iter().enumerate() {
                vec_row.push(pattern[row_index][col_index])
            }
            grid.push(vec_row);
        }

        for (_, row) in grid.iter_mut().enumerate() {
            while row.len() < cols {
                row.push(false);
            }
        }

        while grid.len() < rows {
            let row = vec![false; cols];
            grid.push(row)
        }

        GameOfLife { cols, grid, rows }
    }

    pub fn draw_board(&self) {
        thread::sleep(time::Duration::from_millis(1000));
        println!();
        clearscreen::clear().unwrap();
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("|{}", if self.grid[i][j] { "." } else { " " });
                if j == self.cols - 1 {
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
            if col < self.cols - 1 {
                count_neighbors += self.grid[row - 1][col + 1] as i32;
            }
        }
        if row < self.rows - 1 {
            count_neighbors += self.grid[row + 1][col] as i32;
            if col > 0 {
                count_neighbors += self.grid[row + 1][col - 1] as i32;
            }
            if col < self.cols - 1 {
                count_neighbors += self.grid[row + 1][col + 1] as i32;
            }
        }

        if col > 0 {
            count_neighbors += self.grid[row][col - 1] as i32;
        }
        if col < self.cols - 1 {
            count_neighbors += self.grid[row][col + 1] as i32;
        }

        count_neighbors
    }

    fn is_cell_alive(&self, neighbors: i32, is_alive: bool) -> bool {
        (neighbors == 2 && is_alive) || (neighbors == 3)
    }

    pub fn next_generation(&mut self) {
        let mut temp_board = Vec::new();
        for row in 0..self.rows {
            let mut temp_row = Vec::new();
            for col in 0..self.cols {
                let neighbors = self.get_neighbors(row, col);
                let is_alive = self.grid[row][col];
                temp_row.push(self.is_cell_alive(neighbors, is_alive));
            }
            temp_board.push(temp_row)
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                self.grid[row][col] = temp_board[row][col];
            }
        }
    }
}
