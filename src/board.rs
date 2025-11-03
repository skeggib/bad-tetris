use core::fmt;

#[derive(PartialEq)]
pub struct Board {
    pub cells: [bool; Board::WIDTH * Board::HEIGHT],
}

impl Board {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 10;

    pub fn advance(&mut self) {
        // move all blocks one cell down if the cell bellow is empty
        // iterate through cells from bottom to top to avoid collisions
        let second_to_last_line = Board::HEIGHT - 2; // ignore the most bottom line: blocks on this line cannot fall further
        let first_line = 0;
        for j in (first_line..=second_to_last_line).rev() {
            for i in 0..Board::WIDTH {
                let current_cell = j * Board::WIDTH + i;
                let bellow_cell = current_cell + Board::WIDTH;
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
        for cell in 1..(Board::WIDTH * Board::HEIGHT) {
            // don't move cells on the first column
            if cell % 10 != 0 {
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
        for cell in (0..(Board::WIDTH * Board::HEIGHT - 1)).rev() {
            // don't move cells on the last column
            if cell % 10 != Board::WIDTH - 1 {
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

    fn is_falling(&self, block: usize) -> bool {
        // a block is falling if there is at least one empty cell bellow it
        let mut current = block + Board::WIDTH;
        while current < self.cells.len() {
            if !self.cells[current] {
                return true;
            } else {
                current += Board::WIDTH;
            }
        }
        return false;
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+----------+");
        for i in 0..Board::HEIGHT {
            write!(f, "|");
            for j in 0..Board::WIDTH {
                if self.cells[i * Board::WIDTH + j] {
                    write!(f, "X");
                } else {
                    write!(f, " ");
                }
            }
            writeln!(f, "|");
        }
        write!(f, "+----------+");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advance_moves_block_one_cell_down() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board { cells: [
            X, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let mut expected_board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            X, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        board.advance();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn advance_falling_block_stops_on_bottom() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board { cells: [
            X, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, X, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, X,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let mut expected_board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            X, o, X, o, o, o, o, o, o, X,
        ]};
        for i in 0..11 {
            board.advance();
        }
        assert_eq!(board, expected_board);
    }

    #[test]
    fn advance_falling_block_stops_on_other_block() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board { cells: [
            X, o, o, o, o, o, o, o, o, o,
            o, o, X, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, X,
            o, o, X, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, X,
            o, o, o, o, o, o, o, o, o, X,
            o, o, o, o, o, o, o, o, o, X,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let mut expected_board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, X,
            o, o, o, o, o, o, o, o, o, X,
            o, o, X, o, o, o, o, o, o, X,
            X, o, X, o, o, o, o, o, o, X,
        ]};
        for i in 0..11 {
            board.advance();
        }
        assert_eq!(board, expected_board);
    }

    #[test]
    fn left_moves_blocks_to_the_left() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, X, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, X, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let mut expected_board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            X, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, X, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn left_stops_at_walls() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            X, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, X, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let mut expected_board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            X, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, X, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn left_stops_at_other_blocks() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            X, X, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, X, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let mut expected_board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            X, X, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, X, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn left_only_moves_falling_blocks() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, X, o, o, o, o,
            o, o, o, o, o, X, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, X, o, o, o, o, o, o,
            o, o, o, o, o, o, o, X, o, o,
            o, o, o, X, o, o, o, X, o, o,
        ]};
        #[rustfmt::skip]
        let mut expected_board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, X, o, o, o, o, o,
            o, o, o, o, X, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, X, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, X, o, o,
            o, o, o, X, o, o, o, X, o, o,
        ]};
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_moves_blocks_to_the_right() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, X, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, X, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let mut expected_board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, o, X, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, X, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        board.right();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_stops_at_walls() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, X, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, X,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let mut expected_board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, o, X, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, X,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        board.right();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_stops_at_other_blocks() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, X, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, X, X,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let mut expected_board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, o, X, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, X, X,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
        ]};
        board.right();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_only_moves_falling_blocks() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, X, o, o, o, o,
            o, o, o, o, o, X, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, X, o, o, o, o, o, o,
            o, o, o, o, o, o, o, X, o, o,
            o, o, o, X, o, o, o, X, o, o,
        ]};
        #[rustfmt::skip]
        let mut expected_board = Board { cells: [
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, o, o, X, o, o, o,
            o, o, o, o, o, o, X, o, o, o,
            o, o, o, o, o, o, o, o, o, o,
            o, o, o, o, X, o, o, o, o, o,
            o, o, o, o, o, o, o, X, o, o,
            o, o, o, X, o, o, o, X, o, o,
        ]};
        board.right();
        assert_eq!(board, expected_board);
    }
}
