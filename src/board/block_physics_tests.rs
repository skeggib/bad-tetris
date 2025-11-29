use super::*;

static X: bool = true;
#[allow(non_upper_case_globals)]
static o: bool = false;

const SEED: [u8; 32] = [0; 32];

#[test]
fn advance_moves_block_one_cell_down() {
    #[rustfmt::skip]
    let mut board = Board::<5, 5>::new([
        [X, o, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    #[rustfmt::skip]
    let expected_board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [X, o, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    board.advance();
    assert_eq!(board, expected_board);
}

#[test]
fn advance_falling_block_stops_on_bottom() {
    #[rustfmt::skip]
    let mut board = Board::<5, 5>::new([
        [X, o, o, o, o,],
        [o, o, o, o, o,],
        [o, X, o, o, o,],
        [o, o, o, o, X,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    #[rustfmt::skip]
    let expected_board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
        [X, X, o, o, X,],
    ], rand::rngs::StdRng::from_seed(SEED));
    for _ in 0..4 {
        board.advance();
    }
    assert_eq!(board, expected_board);
}

#[test]
fn advance_falling_block_stops_on_other_block() {
    #[rustfmt::skip]
    let mut board = Board::<5, 5>::new([
        [X, o, X, o, X,],
        [o, o, X, o, X,],
        [o, o, o, o, o,],
        [o, o, o, o, X,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    #[rustfmt::skip]
    let expected_board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, X,],
        [o, o, X, o, X,],
        [X, o, X, o, X,],
    ], rand::rngs::StdRng::from_seed(SEED));
    for _ in 0..4 {
        board.advance();
    }
    assert_eq!(board, expected_board);
}

#[test]
fn left_moves_blocks_to_the_left() {
    #[rustfmt::skip]
    let mut board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [o, X, o, o, o,],
        [o, o, o, X, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    #[rustfmt::skip]
    let expected_board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [X, o, o, o, o,],
        [o, o, X, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    board.left();
    assert_eq!(board, expected_board);
}

#[test]
fn left_stops_at_walls() {
    #[rustfmt::skip]
    let mut board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [X, o, o, o, o,],
        [o, o, o, X, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    #[rustfmt::skip]
    let expected_board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [X, o, o, o, o,],
        [o, o, X, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    board.left();
    assert_eq!(board, expected_board);
}

#[test]
fn left_stops_at_other_blocks() {
    #[rustfmt::skip]
    let mut board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [X, X, o, o, o,],
        [o, o, o, X, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    #[rustfmt::skip]
    let expected_board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [X, X, o, o, o,],
        [o, o, X, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    board.left();
    assert_eq!(board, expected_board);
}

#[test]
fn left_only_moves_falling_blocks() {
    #[rustfmt::skip]
    let mut board = Board::<5, 5>::new([
        [o, o, o, X, o, ],
        [o, o, o, X, o, ],
        [o, X, o, o, o, ],
        [o, o, o, o, X, ],
        [o, X, o, o, X,],
    ], rand::rngs::StdRng::from_seed(SEED));
    #[rustfmt::skip]
    let expected_board = Board::<5, 5>::new([
        [o, o, X, o, o, ],
        [o, o, X, o, o, ],
        [X, o, o, o, o, ],
        [o, o, o, o, X, ],
        [o, X, o, o, X, ],
    ], rand::rngs::StdRng::from_seed(SEED));
    board.left();
    assert_eq!(board, expected_board);
}

#[test]
fn right_moves_blocks_to_the_right() {
    #[rustfmt::skip]
    let mut board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [X, o, o, o, o,],
        [o, o, o, o, o,],
        [o, o, X, o, o,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    #[rustfmt::skip]
    let expected_board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [o, X, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, X, o,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    board.right();
    assert_eq!(board, expected_board);
}

#[test]
fn right_stops_at_walls() {
    #[rustfmt::skip]
    let mut board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [X, o, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, X,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    #[rustfmt::skip]
    let expected_board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [o, X, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, o, X,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    board.right();
    assert_eq!(board, expected_board);
}

#[test]
fn right_stops_at_other_blocks() {
    #[rustfmt::skip]
    let mut board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [X, o, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, X, X,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    #[rustfmt::skip]
    let expected_board = Board::<5, 5>::new([
        [o, o, o, o, o,],
        [o, X, o, o, o,],
        [o, o, o, o, o,],
        [o, o, o, X, X,],
        [o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    board.right();
    assert_eq!(board, expected_board);
}

#[test]
fn right_only_moves_falling_blocks() {
    #[rustfmt::skip]
    let mut board = Board::<5, 5>::new([
        [o, o, X, o, o, ],
        [o, o, X, o, o, ],
        [X, o, o, o, o, ],
        [o, o, o, o, X, ],
        [X, o, o, o, X, ],
    ], rand::rngs::StdRng::from_seed(SEED));
    #[rustfmt::skip]
    let expected_board = Board::<5, 5>::new([
        [o, o, o, X, o, ],
        [o, o, o, X, o, ],
        [o, X, o, o, o, ],
        [o, o, o, o, X, ],
        [X, o, o, o, X, ],
    ], rand::rngs::StdRng::from_seed(SEED));
    board.right();
    assert_eq!(board, expected_board);
}
