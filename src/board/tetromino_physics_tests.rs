use super::*;

static X: Option<Color> = Some(Color::Magenta);
#[allow(non_upper_case_globals)]
static o: Option<Color> = None;

const SEED: [u8; 32] = [30; 32];

#[test]
fn tetromino_spawns_when_all_blocks_have_fallen() {
    // given falling blocks
    #[rustfmt::skip]
    let mut board = Board::<7, 7>::new([
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, X, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));

    // when all blocks have fallen
    board.advance();
    board.advance();

    // and when the board is updated
    board.advance();

    // then a new tetromino spawns
    #[rustfmt::skip]
    let expected = Board::<7, 7>::new([
        [o, o, o, X, o, o, o,],
        [o, o, X, X, X, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, X, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    assert_eq!(board, expected);
}

#[test]
fn advance_moves_tetromino_one_cell_down() {
    // given a newly spawn tetromino
    let mut board = Board::<7, 7>::new([[o; 7]; 7], rand::rngs::StdRng::from_seed(SEED));
    board.advance();
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, X, o, o, o,],
            [o, o, X, X, X, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );

    // when the game advances
    board.advance();

    // then the tetromino moves one cell down
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, X, X, X, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );
}

#[test]
fn left_stops_tetromino_at_walls() {
    // given a tetromino adjacent to the left wall
    let mut board = Board::<7, 7>::new([[o; 7]; 7], rand::rngs::StdRng::from_seed(SEED));
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
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, X, o, o, o, o, o,],
            [X, X, X, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );

    // when the player presses 'left'
    board.left();

    // then the tetromino does not move
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, X, o, o, o, o, o,],
            [X, X, X, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );
}

#[test]
fn right_stops_tetromino_at_walls() {
    // given a tetromino adjacent to the right wall
    let mut board = Board::<7, 7>::new([[o; 7]; 7], rand::rngs::StdRng::from_seed(SEED));
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
                           [o, o, o, o, o, o, o,],
                           [o, o, o, o, o, o, o,],
                           [o, o, o, o, o, o, o,],
                           [o, o, o, o, o, X, o,],
                           [o, o, o, o, X, X, X,],
                           [o, o, o, o, o, o, o,],
                           [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );

    // when the player presses 'right'
    board.right();

    // then the tetromino does not move
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
                           [o, o, o, o, o, o, o,],
                           [o, o, o, o, o, o, o,],
                           [o, o, o, o, o, o, o,],
                           [o, o, o, o, o, X, o,],
                           [o, o, o, o, X, X, X,],
                           [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );
}

#[test]
fn falling_tetromino_dismantles_at_bottom() {
    // given a tetromino adjacent to the bottom wall
    let mut board = Board::<7, 7>::new([[o; 7]; 7], rand::rngs::StdRng::from_seed(SEED));
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
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, o, X, X, o, o,],
            [o, o, o, X, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );

    // when the game advances
    board.advance();

    // then the tetromino dismantles and individual blocks continue falling
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, o, X, X, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );
}

#[test]
fn rotating_a_t() {
    #[rustfmt::skip]
    let mut board = Board::<7, 7>::new([
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
        [o, o, o, o, o, o, o,],
    ], rand::rngs::StdRng::from_seed(SEED));
    board.advance();
    board.advance();

    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, X, X, X, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );

    board.rotate();
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, o, X, X, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );

    board.rotate();
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, X, X, X, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );

    board.rotate();
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, X, X, o, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );

    board.rotate();
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, X, X, X, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );
}

#[test]
fn rotating_a_tetromino_adjacent_to_left_wall_making_it_overflow_the_grid() {
    // given a tetromino adjacent to a wall
    let mut board = Board::<7, 7>::new([[None; 7]; 7], rand::rngs::StdRng::from_seed(SEED));
    board.advance();
    board.advance();
    board.rotate();
    board.left();
    board.left();
    board.left();
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [X, o, o, o, o, o, o,],
            [X, X, o, o, o, o, o,],
            [X, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );

    // when a rotation makes it overflow the grid
    board.rotate();

    // then it is moved back to the limits of the grid
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [X, X, X, o, o, o, o,],
            [o, X, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );
}

#[test]
fn rotating_a_tetromino_adjacent_to_right_wall_making_it_overflow_the_grid() {
    // given a tetromino adjacent to a wall
    let mut board = Board::<7, 7>::new([[None; 7]; 7], rand::rngs::StdRng::from_seed(SEED));
    board.advance();
    board.advance();
    board.rotate();
    board.rotate();
    board.rotate();
    board.right();
    board.right();
    board.right();
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, X,],
            [o, o, o, o, o, X, X,],
            [o, o, o, o, o, o, X,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );

    // when a rotation makes it overflow the grid
    board.rotate();

    // then it is moved back to the limits of the grid
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, X, o,],
            [o, o, o, o, X, X, X,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );
}

#[test]
fn tetromino_cannot_be_rotated_on_the_ground() {
    // given a tetromino that just moved to the ground
    let mut board = Board::<7, 7>::new([[None; 7]; 7], rand::rngs::StdRng::from_seed(SEED));
    board.advance();
    board.advance();
    board.advance();
    board.advance();
    board.advance();
    board.advance();
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, X, X, X, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );

    // when the tetromino is rotated
    board.rotate();

    // then it does nothing because it has already been dismantled
    assert_eq!(
        board,
        #[rustfmt::skip]
        Board::<7, 7>::new([
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, o, o, o, o,],
            [o, o, o, X, o, o, o,],
            [o, o, X, X, X, o, o,],
        ], rand::rngs::StdRng::from_seed(SEED))
    );
}
