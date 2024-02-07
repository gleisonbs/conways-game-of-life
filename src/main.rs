mod engine;

use engine::game_of_life::GameOfLife;
use engine::patterns::Pattern;

fn main() {
    let glider = Pattern::glider();
    let mut game_of_life = GameOfLife::new(glider);
    loop {
        game_of_life.draw_board();
        game_of_life.next_generation();
    }
}
