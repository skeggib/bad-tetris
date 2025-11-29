use crate::tetrominos;
use core::fmt;
use rand::prelude::*;

pub type Block = Option<Color>;

pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[Block; WIDTH]; HEIGHT],
    tetromino: Option<TetrominoPosition>,
    rng: rand::rngs::StdRng,
}

struct TetrominoPosition {
    index: usize,
    col: isize, // isize because tetromino position can be negative when adgacent to the left wall
    row: isize,
    orientation: usize,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Color {
    Cyan,
    Blue,
    Magenta,
    Yellow,
    Orange,
    Green,
    Red,
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    pub fn new(cells: [[Block; WIDTH]; HEIGHT], rng: rand::rngs::StdRng) -> Board<WIDTH, HEIGHT> {
        return Board::<WIDTH, HEIGHT> {
            cells: cells,
            tetromino: None,
            rng: rng,
        };
    }

    const TETROMINOS: [([[[bool; 4]; 4]; 4], Color); 7] = [
        (tetrominos::TETROMINO_I, Color::Cyan),
        (tetrominos::TETROMINO_O, Color::Blue),
        (tetrominos::TETROMINO_T, Color::Magenta),
        (tetrominos::TETROMINO_L, Color::Yellow),
        (tetrominos::TETROMINO_J, Color::Orange),
        (tetrominos::TETROMINO_S, Color::Green),
        (tetrominos::TETROMINO_Z, Color::Red),
    ];

    pub fn cells(&self) -> [[Block; WIDTH]; HEIGHT] {
        if let Some(tetromino) = &self.tetromino {
            Board::<WIDTH, HEIGHT>::add_tetromino(
                self.cells,
                tetromino.row,
                tetromino.col,
                (
                    Board::<WIDTH, HEIGHT>::TETROMINOS[tetromino.index].0[tetromino.orientation],
                    Board::<WIDTH, HEIGHT>::TETROMINOS[tetromino.index].1,
                ),
            )
        } else {
            self.cells.clone()
        }
    }

    pub fn advance(&mut self) {
        if self.falling_blocks() || self.falling_tetromino() {
            self.down();
        } else {
            self.spawn_tetromino();
        }
    }

    fn falling_blocks(&self) -> bool {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if self.cells[row][col] != None && self.is_falling(row, col) {
                    return true;
                }
            }
        }
        return false;
    }

    fn falling_tetromino(&self) -> bool {
        match self.tetromino {
            Some(_) => true,
            None => false,
        }
    }

    fn spawn_tetromino(&mut self) {
        let index = self.rng.next_u32() as usize % Board::<WIDTH, HEIGHT>::TETROMINOS.len();
        let t_width = Board::<WIDTH, HEIGHT>::TETROMINOS[index].0[0][0].len();
        let start = WIDTH / 2 - t_width / 2;
        self.tetromino = Some(TetrominoPosition {
            index: index,
            col: start as isize,
            row: 0,
            orientation: 0,
        });
    }

    fn add_tetromino<const TETROMINO_WIDTH: usize, const TETROMINO_HEIGHT: usize>(
        cells: [[Block; WIDTH]; HEIGHT],
        row: isize,
        col: isize,
        tetromino: ([[bool; TETROMINO_WIDTH]; TETROMINO_HEIGHT], Color),
    ) -> [[Block; WIDTH]; HEIGHT] {
        let mut result = cells.clone();
        for t_row in 0..TETROMINO_HEIGHT {
            for t_col in 0..TETROMINO_WIDTH {
                let b_row = row + t_row as isize;
                let b_col = col + t_col as isize;
                if b_row >= 0 && b_col >= 0 && b_row < HEIGHT as isize && b_col < WIDTH as isize {
                    let (t_values, t_color) = tetromino;
                    if t_values[t_row][t_col] {
                        result[b_row as usize][b_col as usize] = Some(t_color);
                    }
                }
            }
        }
        return result;
    }

    fn down(&mut self) {
        self.tetromino_down();
        self.blocks_down();
        if !self.is_tetromino_falling() {
            self.dismantle_tetromino();
        }
    }

    fn tetromino_down(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            tetromino.row += 1;
        }
    }

    fn dismantle_tetromino(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            self.cells = Board::add_tetromino(
                self.cells,
                tetromino.row as isize,
                tetromino.col as isize,
                (
                    Board::<WIDTH, HEIGHT>::TETROMINOS[tetromino.index].0[tetromino.orientation],
                    Board::<WIDTH, HEIGHT>::TETROMINOS[tetromino.index].1,
                ),
            );
            self.tetromino = None;
        }
    }

    fn blocks_down(&mut self) {
        // move all blocks one cell down if the cell bellow is empty
        // iterate through cells from bottom to top to avoid collisions
        let second_to_last_line = HEIGHT - 2; // ignore the most bottom line: blocks on this line cannot fall further
        let first_line = 0;
        for row in (first_line..=second_to_last_line).rev() {
            for col in 0..WIDTH {
                let current_cell_empty = self.cells[row][col] == None;
                let bellow_cell_empty = self.cells[row + 1][col] == None;

                if !current_cell_empty && bellow_cell_empty {
                    self.cells[row + 1][col] = self.cells[row][col];
                    self.cells[row][col] = None;
                }
            }
        }
    }

    pub fn left(&mut self) {
        self.tetromino_left();
        self.blocks_left();
    }

    fn tetromino_left(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            let current_tetromino =
                &Board::<WIDTH, HEIGHT>::TETROMINOS[tetromino.index].0[tetromino.orientation];
            let t_width = current_tetromino[0].len();
            let t_height = current_tetromino.len();

            // find column of most left block of tetromino
            let mut most_left = t_width;
            for col in 0..t_width {
                for row in 0..t_height {
                    if current_tetromino[row][col] {
                        most_left = std::cmp::min(most_left, col);
                        break;
                    }
                }
            }
            // move tetromino left if it does not touch the wall
            let actual_tetromino_position = tetromino.col + most_left as isize;
            assert!(actual_tetromino_position >= 0);
            if actual_tetromino_position > 0 {
                tetromino.col -= 1;
            }
        }
    }

    fn blocks_left(&mut self) {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                // don't move cells on the first column
                if col != 0 {
                    // move only falling cells
                    if self.is_falling(row, col) {
                        // if the current cell is not empty and the left cell is
                        if self.cells[row][col] != None && self.cells[row][col - 1] == None {
                            // move the cell to the left
                            self.cells[row][col - 1] = self.cells[row][col];
                            self.cells[row][col] = None;
                        }
                    }
                }
            }
        }
    }

    pub fn right(&mut self) {
        self.tetromino_right();
        self.blocks_right();
    }

    fn tetromino_right(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            let current_tetromino =
                &Board::<WIDTH, HEIGHT>::TETROMINOS[tetromino.index].0[tetromino.orientation];
            let t_width = current_tetromino[0].len();
            let t_height = current_tetromino.len();

            // find column of most right block of tetromino
            let mut most_right: usize = 0;
            for col in (0..t_width).rev() {
                for row in 0..t_height {
                    if current_tetromino[row][col] {
                        most_right = std::cmp::max(most_right, col);
                        break;
                    }
                }
            }
            // move tetromino left if it does not touch the wall
            let actual_tetromino_position = tetromino.col + most_right as isize;
            assert!(actual_tetromino_position >= 0);
            if (actual_tetromino_position as usize) < WIDTH - 1 {
                tetromino.col += 1;
            }
        }
    }

    fn blocks_right(&mut self) {
        for row in 0..HEIGHT {
            for col in (0..WIDTH).rev() {
                // don't move cells on the last column
                if col != WIDTH - 1 {
                    // move only falling cells
                    if self.is_falling(row, col) {
                        // if the current cell is not empty and the left cell is
                        if self.cells[row][col] != None && self.cells[row][col + 1] == None {
                            // move the cell to the left
                            self.cells[row][col + 1] = self.cells[row][col];
                            self.cells[row][col] = None;
                        }
                    }
                }
            }
        }
    }

    pub fn rotate(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            tetromino.orientation = (tetromino.orientation + 1) % 4;

            let current_tetromino =
                &Board::<WIDTH, HEIGHT>::TETROMINOS[tetromino.index].0[tetromino.orientation];
            let t_width = current_tetromino[0].len();
            let t_height = current_tetromino.len();

            // find column of most left block of tetromino
            let mut most_left = t_width;
            for col in 0..t_width {
                for row in 0..t_height {
                    if current_tetromino[row][col] {
                        most_left = std::cmp::min(most_left, col);
                        break;
                    }
                }
            }

            // find column of most right block of tetromino
            let mut most_right: usize = 0;
            for col in (0..t_width).rev() {
                for row in 0..t_height {
                    if current_tetromino[row][col] {
                        most_right = std::cmp::max(most_right, col);
                        break;
                    }
                }
            }

            if (tetromino.col + most_left as isize) < 0 {
                tetromino.col -= tetromino.col + most_left as isize;
            }

            if (tetromino.col + most_right as isize) >= WIDTH as isize {
                tetromino.col -= WIDTH as isize - (tetromino.col + most_right as isize) + 1;
            }
        }
    }

    fn is_falling(&self, row: usize, col: usize) -> bool {
        // a block is falling if there is at least one empty cell bellow it
        for row in (row + 1)..HEIGHT {
            if self.cells[row][col] == None {
                return true;
            }
        }
        return false;
    }

    fn is_tetromino_falling(&self) -> bool {
        // a tetromino is falling if all its blocks are falling
        if let Some(tetromino) = &self.tetromino {
            let current_tetromino =
                &Board::<WIDTH, HEIGHT>::TETROMINOS[tetromino.index].0[tetromino.orientation];
            let t_width = current_tetromino[0].len();
            let t_height = current_tetromino.len();

            for col in 0..t_width {
                for row in 0..t_height {
                    if current_tetromino[row][col] {
                        if !self.is_falling(
                            (tetromino.row + row as isize) as usize,
                            (tetromino.col + col as isize) as usize,
                        ) {
                            return false;
                        }
                    }
                }
            }
            return true;
        } else {
            false
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Debug for Board<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cells = self.cells();
        writeln!(f, "+----------+")?;
        for row in 0..HEIGHT {
            write!(f, "|")?;
            for col in 0..WIDTH {
                if cells[row][col] != None {
                    write!(f, "X")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "|")?;
        }
        write!(f, "+----------+")?;
        Ok(())
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> PartialEq for Board<WIDTH, HEIGHT> {
    fn eq(&self, other: &Self) -> bool {
        self.cells() == other.cells()
    }
}

#[cfg(test)]
mod block_physics_tests;

#[cfg(test)]
mod tetromino_physics_tests;
