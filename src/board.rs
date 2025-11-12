use core::fmt;

pub struct Board<const WIDTH: usize, const HEIGHT: usize>
where
    [(); WIDTH * HEIGHT]:,
{
    cells: [bool; WIDTH * HEIGHT],
    tetromino: Option<TetrominoPosition>,
}

struct TetrominoPosition {
    position: isize, // isize because tetromino position can be negative when adgacent to the left wall
    orientation: usize,
}

static X: bool = true;
#[allow(non_upper_case_globals)]
static o: bool = false;

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    pub fn new(cells: [bool; WIDTH * HEIGHT]) -> Board<WIDTH, HEIGHT> {
        return Board::<WIDTH, HEIGHT> {
            cells: cells,
            tetromino: None,
        };
    }

    const TETROMINO_WIDTH: usize = 4;
    const TETROMINO_HEIGHT: usize = 4;

    #[rustfmt::skip]
    const TETROMINO_T: [[[bool; 4]; 4]; 4] = [
       [[o, o, X, o],
        [o, X, X, X],
        [o, o, o, o],
        [o, o, o, o]],

       [[o, o, X, o],
        [o, o, X, X],
        [o, o, X, o],
        [o, o, o, o]],

       [[o, o, o, o],
        [o, X, X, X],
        [o, o, X, o],
        [o, o, o, o]],

       [[o, o, X, o],
        [o, X, X, o],
        [o, o, X, o],
        [o, o, o, o]],
    ];

    pub fn cells(&self) -> [bool; WIDTH * HEIGHT] {
        if let Some(tetromino) = &self.tetromino {
            Board::add_tetromino(
                self.cells,
                tetromino.position,
                Board::TETROMINO_T[tetromino.orientation],
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
        for cell in 0..(WIDTH * HEIGHT) {
            if self.cells[cell] && self.is_falling(cell) {
                return true;
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
        let start = WIDTH / 2 - Board::TETROMINO_WIDTH / 2;
        self.tetromino = Some(TetrominoPosition {
            position: start as isize,
            orientation: 0,
        });
    }

    fn add_tetromino<const TETROMINO_WIDTH: usize, const TETROMINO_HEIGHT: usize>(
        cells: [bool; WIDTH * HEIGHT],
        position: isize,
        tetromino: [[bool; TETROMINO_WIDTH]; TETROMINO_HEIGHT],
    ) -> [bool; WIDTH * HEIGHT] {
        let mut result = cells.clone();
        for line in 0..TETROMINO_HEIGHT {
            for col in 0..TETROMINO_WIDTH {
                let position = (position + (col + line * WIDTH) as isize) as usize;
                if position < WIDTH * HEIGHT {
                    result[position] |= tetromino[line][col];
                }
            }
        }
        return result;
    }

    fn down(&mut self) {
        self.tetromino_down();
        self.blocks_down();
    }

    fn tetromino_down(&mut self) {
        let is_tetromino_falling = self.is_tetromino_falling();
        if let Some(tetromino) = &mut self.tetromino {
            if is_tetromino_falling {
                tetromino.position += WIDTH as isize;
            } else {
                self.dismantle_tetromino();
            }
        }
    }

    fn dismantle_tetromino(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            self.cells = Board::add_tetromino(
                self.cells,
                tetromino.position,
                Board::TETROMINO_T[tetromino.orientation],
            );
            self.tetromino = None;
        }
    }

    fn blocks_down(&mut self) {
        // move all blocks one cell down if the cell bellow is empty
        // iterate through cells from bottom to top to avoid collisions
        let second_to_last_line = HEIGHT - 2; // ignore the most bottom line: blocks on this line cannot fall further
        let first_line = 0;
        for j in (first_line..=second_to_last_line).rev() {
            for i in 0..WIDTH {
                let current_cell = j * WIDTH + i;
                let bellow_cell = current_cell + WIDTH;
                let current_cell_empty = !self.cells[current_cell];
                let bellow_cell_empty = !self.cells[bellow_cell];

                if !current_cell_empty && bellow_cell_empty {
                    self.cells[current_cell] = false;
                    self.cells[bellow_cell] = true;
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
            // find column of most left block of tetromino
            let mut most_left = Board::TETROMINO_WIDTH;
            for col in 0..Board::TETROMINO_WIDTH {
                for row in 0..Board::TETROMINO_HEIGHT {
                    if Board::TETROMINO_T[tetromino.orientation][row][col] {
                        most_left = std::cmp::min(most_left, col);
                        break;
                    }
                }
            }
            // move tetromino left if it does not touch the wall
            let actual_tetromino_position = tetromino.position + most_left as isize;
            assert!(actual_tetromino_position >= 0);
            if actual_tetromino_position as usize % WIDTH > 0 {
                tetromino.position -= 1;
            }
        }
    }

    fn blocks_left(&mut self) {
        for cell in 1..(WIDTH * HEIGHT) {
            // don't move cells on the first column
            if cell % WIDTH != 0 {
                // move only falling cells
                if self.is_falling(cell) {
                    // if the current cell is not empty and the left cell is
                    if self.cells[cell] && !self.cells[cell - 1] {
                        // move the cell to the left
                        self.cells[cell] = false;
                        self.cells[cell - 1] = true;
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
            // find column of most right block of tetromino
            let mut most_right: usize = 0;
            for col in (0..Board::TETROMINO_WIDTH).rev() {
                for row in 0..Board::TETROMINO_HEIGHT {
                    if Board::TETROMINO_T[tetromino.orientation][row][col] {
                        most_right = std::cmp::max(most_right, col);
                        break;
                    }
                }
            }
            // move tetromino left if it does not touch the wall
            let actual_tetromino_position = tetromino.position + most_right as isize;
            assert!(actual_tetromino_position >= 0);
            if actual_tetromino_position as usize % WIDTH < WIDTH - 1 {
                tetromino.position += 1;
            }
        }
    }

    fn blocks_right(&mut self) {
        for cell in (0..(WIDTH * HEIGHT - 1)).rev() {
            // don't move cells on the last column
            if cell % WIDTH != WIDTH - 1 {
                // move only falling cells
                if self.is_falling(cell) {
                    // if the current cell is not empty and the left cell is
                    if self.cells[cell] && !self.cells[cell + 1] {
                        // move the cell to the left
                        self.cells[cell] = false;
                        self.cells[cell + 1] = true;
                    }
                }
            }
        }
    }

    pub fn rotate(&mut self) {
        if let Some(tetromino) = &mut self.tetromino {
            tetromino.orientation = (tetromino.orientation + 1) % 4;
        }
    }

    fn is_falling(&self, block: usize) -> bool {
        // a block is falling if there is at least one empty cell bellow it
        let mut current = block + WIDTH;
        while current < self.cells.len() {
            if !self.cells[current] {
                return true;
            } else {
                current += WIDTH;
            }
        }
        return false;
    }

    fn is_tetromino_falling(&self) -> bool {
        // a tetromino is falling if all its blocks are falling
        if let Some(tetromino) = &self.tetromino {
            for col in 0..Board::TETROMINO_WIDTH {
                for row in 0..Board::TETROMINO_HEIGHT {
                    if Board::TETROMINO_T[tetromino.orientation][row][col] {
                        let position = (tetromino.position + (col + row * WIDTH) as isize) as usize;
                        if !self.is_falling(position) {
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

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Debug for Board<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cells = self.cells();
        writeln!(f, "+----------+")?;
        for i in 0..HEIGHT {
            write!(f, "|")?;
            for j in 0..WIDTH {
                if cells[i * WIDTH + j] {
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

impl<const WIDTH: usize, const HEIGHT: usize> PartialEq for Board<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    fn eq(&self, other: &Self) -> bool {
        self.cells() == other.cells()
    }
}

#[cfg(test)]
mod block_physics_tests {
    use super::*;

    #[test]
    fn advance_moves_block_one_cell_down() {
        #[rustfmt::skip]
        let mut board = Board::<5, 5>::new([
            X, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]);
        #[rustfmt::skip]
        let expected_board = Board::<5, 5>::new([
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]);
        board.advance();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn advance_falling_block_stops_on_bottom() {
        #[rustfmt::skip]
        let mut board = Board::<5, 5>::new([
            X, o, o, o, o,
            o, o, o, o, o,
            o, X, o, o, o,
            o, o, o, o, X,
            o, o, o, o, o,
        ]);
        #[rustfmt::skip]
        let expected_board = Board::<5, 5>::new([
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
            X, X, o, o, X,
        ]);
        for _ in 0..4 {
            board.advance();
        }
        assert_eq!(board, expected_board);
    }

    #[test]
    fn advance_falling_block_stops_on_other_block() {
        #[rustfmt::skip]
        let mut board = Board::<5, 5>::new([
            X, o, X, o, X,
            o, o, X, o, X,
            o, o, o, o, o,
            o, o, o, o, X,
            o, o, o, o, o,
        ]);
        #[rustfmt::skip]
        let expected_board = Board::<5, 5>::new([
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, X,
            o, o, X, o, X,
            X, o, X, o, X,
        ]);
        for _ in 0..4 {
            board.advance();
        }
        assert_eq!(board, expected_board);
    }

    #[test]
    fn left_moves_blocks_to_the_left() {
        #[rustfmt::skip]
        let mut board = Board::<5, 5>::new([
            o, o, o, o, o,
            o, X, o, o, o,
            o, o, o, X, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]);
        #[rustfmt::skip]
        let expected_board = Board::<5, 5>::new([
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, X, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]);
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn left_stops_at_walls() {
        #[rustfmt::skip]
        let mut board = Board::<5, 5>::new([
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, o, X, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]);
        #[rustfmt::skip]
        let expected_board = Board::<5, 5>::new([
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, X, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]);
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn left_stops_at_other_blocks() {
        #[rustfmt::skip]
        let mut board = Board::<5, 5>::new([
            o, o, o, o, o,
            X, X, o, o, o,
            o, o, o, X, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]);
        #[rustfmt::skip]
        let expected_board = Board::<5, 5>::new([
            o, o, o, o, o,
            X, X, o, o, o,
            o, o, X, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]);
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn left_only_moves_falling_blocks() {
        #[rustfmt::skip]
        let mut board = Board::<5, 5>::new([
            o, o, o, X, o, 
            o, o, o, X, o, 
            o, X, o, o, o, 
            o, o, o, o, X, 
            o, X, o, o, X,
        ]);
        #[rustfmt::skip]
        let expected_board = Board::<5, 5>::new([
            o, o, X, o, o, 
            o, o, X, o, o, 
            X, o, o, o, o, 
            o, o, o, o, X, 
            o, X, o, o, X, 
        ]);
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_moves_blocks_to_the_right() {
        #[rustfmt::skip]
        let mut board = Board::<5, 5>::new([
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, o, o, o,
            o, o, X, o, o,
            o, o, o, o, o,
        ]);
        #[rustfmt::skip]
        let expected_board = Board::<5, 5>::new([
            o, o, o, o, o,
            o, X, o, o, o,
            o, o, o, o, o,
            o, o, o, X, o,
            o, o, o, o, o,
        ]);
        board.right();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_stops_at_walls() {
        #[rustfmt::skip]
        let mut board = Board::<5, 5>::new([
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, X,
            o, o, o, o, o,
        ]);
        #[rustfmt::skip]
        let expected_board = Board::<5, 5>::new([
            o, o, o, o, o,
            o, X, o, o, o,
            o, o, o, o, o,
            o, o, o, o, X,
            o, o, o, o, o,
        ]);
        board.right();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_stops_at_other_blocks() {
        #[rustfmt::skip]
        let mut board = Board::<5, 5>::new([
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, o, o, o,
            o, o, o, X, X,
            o, o, o, o, o,
        ]);
        #[rustfmt::skip]
        let expected_board = Board::<5, 5>::new([
            o, o, o, o, o,
            o, X, o, o, o,
            o, o, o, o, o,
            o, o, o, X, X,
            o, o, o, o, o,
        ]);
        board.right();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_only_moves_falling_blocks() {
        #[rustfmt::skip]
        let mut board = Board::<5, 5>::new([
            o, o, X, o, o, 
            o, o, X, o, o, 
            X, o, o, o, o, 
            o, o, o, o, X, 
            X, o, o, o, X, 
        ]);
        #[rustfmt::skip]
        let expected_board = Board::<5, 5>::new([
            o, o, o, X, o, 
            o, o, o, X, o, 
            o, X, o, o, o, 
            o, o, o, o, X, 
            X, o, o, o, X, 
        ]);
        board.right();
        assert_eq!(board, expected_board);
    }
}

#[cfg(test)]
mod tetromino_physics_tests {
    use super::*;

    #[test]
    fn tetromino_spawns_when_all_blocks_have_fallen() {
        // given falling blocks
        #[rustfmt::skip]
        let mut board = Board::<7, 7>::new([
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, X, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
        ]);

        // when all blocks have fallen
        board.advance();
        board.advance();

        // and when the board is updated
        board.advance();

        // then a new tetromino spawns
        #[rustfmt::skip]
        let expected = Board::<7, 7>::new([
            o, o, o, X, o, o, o,
            o, o, X, X, X, o, o,
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, X, o, o, o, o,
        ]);
        assert_eq!(board, expected);
    }

    #[test]
    fn advance_moves_tetromino_one_cell_down() {
        // given a newly spawn tetromino
        let mut board = Board::<7, 7>::new([o; 7 * 7]);
        board.advance();
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                o, o, o, X, o, o, o,
                o, o, X, X, X, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
            ])
        );

        // when the game advances
        board.advance();

        // then the tetromino moves one cell down
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                o, o, o, o, o, o, o,
                o, o, o, X, o, o, o,
                o, o, X, X, X, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
            ])
        );
    }

    #[test]
    fn left_stops_tetromino_at_walls() {
        // given a tetromino adjacent to the left wall
        let mut board = Board::<7, 7>::new([o; 7 * 7]);
        board.advance();
        board.advance();
        board.advance();
        board.advance();
        board.left();
        board.left();
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, X, o, o, o, o, o,
                X, X, X, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
            ])
        );

        // when the player presses 'left'
        board.left();

        // then the tetromino does not move
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, X, o, o, o, o, o,
                X, X, X, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
            ])
        );
    }

    #[test]
    fn right_stops_tetromino_at_walls() {
        // given a tetromino adjacent to the right wall
        let mut board = Board::<7, 7>::new([o; 7 * 7]);
        board.advance();
        board.advance();
        board.advance();
        board.advance();
        board.right();
        board.right();
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                               o, o, o, o, o, o, o,
                               o, o, o, o, o, o, o,
                               o, o, o, o, o, o, o,
                               o, o, o, o, o, X, o,
                               o, o, o, o, X, X, X,
                               o, o, o, o, o, o, o,
                               o, o, o, o, o, o, o,
            ])
        );

        // when the player presses 'right'
        board.right();

        // then the tetromino does not move
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                               o, o, o, o, o, o, o,
                               o, o, o, o, o, o, o,
                               o, o, o, o, o, o, o,
                               o, o, o, o, o, X, o,
                               o, o, o, o, X, X, X,
                               o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
            ])
        );
    }

    #[test]
    fn falling_tetromino_dismantles_at_bottom() {
        // given a tetromino adjacent to the bottom wall
        let mut board = Board::<7, 7>::new([o; 7 * 7]);
        board.advance();
        board.advance();
        board.rotate();
        board.advance();
        board.advance();
        board.advance();
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, X, o, o, o,
                o, o, o, X, X, o, o,
                o, o, o, X, o, o, o,
            ])
        );

        // when the game advances
        board.advance();

        // then the tetromino dismantles and individual blocks continue falling
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, X, o, o, o,
                o, o, o, X, o, o, o,
                o, o, o, X, X, o, o,
            ])
        );
    }

    #[test]
    fn roting_a_t() {
        #[rustfmt::skip]
        let mut board = Board::<7, 7>::new([
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
            o, o, o, o, o, o, o,
        ]);
        board.advance();
        board.advance();

        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                o, o, o, o, o, o, o,
                o, o, o, X, o, o, o,
                o, o, X, X, X, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
            ])
        );

        board.rotate();
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                o, o, o, o, o, o, o,
                o, o, o, X, o, o, o,
                o, o, o, X, X, o, o,
                o, o, o, X, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
            ])
        );

        board.rotate();
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, X, X, X, o, o,
                o, o, o, X, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
            ])
        );

        board.rotate();
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                o, o, o, o, o, o, o,
                o, o, o, X, o, o, o,
                o, o, X, X, o, o, o,
                o, o, o, X, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
            ])
        );

        board.rotate();
        assert_eq!(
            board,
            #[rustfmt::skip]
            Board::<7, 7>::new([
                o, o, o, o, o, o, o,
                o, o, o, X, o, o, o,
                o, o, X, X, X, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
                o, o, o, o, o, o, o,
            ])
        );
    }
}
