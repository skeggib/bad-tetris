use core::fmt;

#[derive(PartialEq)]
pub struct Board<const WIDTH: usize, const HEIGHT: usize>
where
    [(); WIDTH * HEIGHT]:,
{
    pub cells: [bool; WIDTH * HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    pub fn advance(&mut self) {
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
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Debug for Board<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+----------+");
        for i in 0..HEIGHT {
            write!(f, "|");
            for j in 0..WIDTH {
                if self.cells[i * WIDTH + j] {
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
        let mut board = Board::<5, 5> { cells: [
            X, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let expected_board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]};
        board.advance();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn advance_falling_block_stops_on_bottom() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board::<5, 5> { cells: [
            X, o, o, o, o,
            o, o, o, o, o,
            o, X, o, o, o,
            o, o, o, o, X,
            o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let expected_board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
            X, X, o, o, X,
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
        let mut board = Board::<5, 5> { cells: [
            X, o, X, o, X,
            o, o, X, o, X,
            o, o, o, o, o,
            o, o, o, o, X,
            o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let expected_board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, X,
            o, o, X, o, X,
            X, o, X, o, X,
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
        let mut board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            o, X, o, o, o,
            o, o, o, X, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let expected_board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, X, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]};
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn left_stops_at_walls() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, o, X, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let expected_board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, X, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]};
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn left_stops_at_other_blocks() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            X, X, o, o, o,
            o, o, o, X, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let expected_board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            X, X, o, o, o,
            o, o, X, o, o,
            o, o, o, o, o,
            o, o, o, o, o,
        ]};
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn left_only_moves_falling_blocks() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board::<5, 5> { cells: [
            o, o, o, X, o, 
            o, o, o, X, o, 
            o, X, o, o, o, 
            o, o, o, o, X, 
            o, X, o, o, X,
        ]};
        #[rustfmt::skip]
        let expected_board = Board::<5, 5> { cells: [
            o, o, X, o, o, 
            o, o, X, o, o, 
            X, o, o, o, o, 
            o, o, o, o, X, 
            o, X, o, o, X, 
        ]};
        board.left();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_moves_blocks_to_the_right() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, o, o, o,
            o, o, X, o, o,
            o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let expected_board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            o, X, o, o, o,
            o, o, o, o, o,
            o, o, o, X, o,
            o, o, o, o, o,
        ]};
        board.right();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_stops_at_walls() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, o, o, o,
            o, o, o, o, X,
            o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let expected_board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            o, X, o, o, o,
            o, o, o, o, o,
            o, o, o, o, X,
            o, o, o, o, o,
        ]};
        board.right();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_stops_at_other_blocks() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            X, o, o, o, o,
            o, o, o, o, o,
            o, o, o, X, X,
            o, o, o, o, o,
        ]};
        #[rustfmt::skip]
        let expected_board = Board::<5, 5> { cells: [
            o, o, o, o, o,
            o, X, o, o, o,
            o, o, o, o, o,
            o, o, o, X, X,
            o, o, o, o, o,
        ]};
        board.right();
        assert_eq!(board, expected_board);
    }

    #[test]
    fn right_only_moves_falling_blocks() {
        let X = true;
        let o = false;
        #[rustfmt::skip]
        let mut board = Board::<5, 5> { cells: [
            o, o, X, o, o, 
            o, o, X, o, o, 
            X, o, o, o, o, 
            o, o, o, o, X, 
            X, o, o, o, X, 
        ]};
        #[rustfmt::skip]
        let expected_board = Board::<5, 5> { cells: [
            o, o, o, X, o, 
            o, o, o, X, o, 
            o, X, o, o, o, 
            o, o, o, o, X, 
            X, o, o, o, X, 
        ]};
        board.right();
        assert_eq!(board, expected_board);
    }
}
