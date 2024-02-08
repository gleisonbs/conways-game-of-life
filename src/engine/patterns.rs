pub struct Pattern {}

impl Pattern {
    pub fn none() -> Vec<Vec<bool>> {
        let pattern = vec!["."];
        self::Pattern::translate_pattern(pattern)
    }

    pub fn _blinker() -> Vec<Vec<bool>> {
        let pattern = vec![".....", "..O..", "..O..", "..O..", "....."];
        self::Pattern::translate_pattern(pattern)
    }

    pub fn _glider() -> Vec<Vec<bool>> {
        let pattern = vec!["..O..", "...O.", ".OOO.."];
        self::Pattern::translate_pattern(pattern)
    }

    pub fn _weekender() -> Vec<Vec<bool>> {
        let pattern = vec![
            ".O............O.......O............O.",
            ".O............O.......O............O.",
            "O.O..........O.O.....O.O..........O.O",
            ".O............O.......O............O.",
            ".O............O.......O............O.",
            "..O...OOOO...O.........O...OOOO...O..",
            "......OOOO........O........OOOO......",
            "..OOOO....OOOO..OOOOO..OOOO....OOOO..",
            "...............O.O.O.O...............",
            "....O......O.............O......O....",
            ".....OO..OO......O.O......OO..OO.....",
            "..............O..O.O..O..............",
            "..............O..O.O..O..............",
            "..............O.O...O.O..............",
            ".....................................",
            ".....................................",
            "..............OO.....OO..............",
            "..............OO.....OO..............",
            ".....................................",
            "................OO.OO................",
            "...............O.O.O.O...............",
            "...............O.O.O.O...............",
            ".................O.O.................",
            "...............OO...OO................",
        ];
        self::Pattern::translate_pattern(pattern)
    }

    fn translate_pattern(pattern: Vec<&str>) -> Vec<Vec<bool>> {
        let mut matrix: Vec<Vec<bool>> = Vec::new();
        for row in pattern {
            let mut bool_row = Vec::new();
            for char in row.chars() {
                bool_row.push(char != '.');
            }
            matrix.push(bool_row);
        }

        matrix
    }
}
