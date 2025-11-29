const X: bool = true;
#[allow(non_upper_case_globals)]
const o: bool = false;
#[rustfmt::skip]

pub const TETROMINO_I: [[[bool; 4]; 4]; 4] = [
   [[o, o, o, o],
    [X, X, X, X],
    [o, o, o, o],
    [o, o, o, o]],

   [[o, o, X, o],
    [o, o, X, o],
    [o, o, X, o],
    [o, o, X, o]],

   [[o, o, o, o],
    [o, o, o, o],
    [X, X, X, X],
    [o, o, o, o]],

   [[o, X, o, o],
    [o, X, o, o],
    [o, X, o, o],
    [o, X, o, o]],
];

#[rustfmt::skip]
pub const TETROMINO_O: [[[bool; 4]; 4]; 4] = [
   [[o, o, o, o],
    [o, X, X, o],
    [o, X, X, o],
    [o, o, o, o]],

   [[o, o, o, o],
    [o, X, X, o],
    [o, X, X, o],
    [o, o, o, o]],

   [[o, o, o, o],
    [o, X, X, o],
    [o, X, X, o],
    [o, o, o, o]],

   [[o, o, o, o],
    [o, X, X, o],
    [o, X, X, o],
    [o, o, o, o]],
];

#[rustfmt::skip]
pub const TETROMINO_T: [[[bool; 4]; 4]; 4] = [
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

#[rustfmt::skip]
pub const TETROMINO_L: [[[bool; 4]; 4]; 4] = [
   [[o, o, o, X],
    [o, X, X, X],
    [o, o, o, o],
    [o, o, o, o]],

   [[o, o, X, o],
    [o, o, X, o],
    [o, o, X, X],
    [o, o, o, o]],

   [[o, o, o, o],
    [o, X, X, X],
    [o, X, o, o],
    [o, o, o, o]],

   [[o, X, X, o],
    [o, o, X, o],
    [o, o, X, o],
    [o, o, o, o]],
];

#[rustfmt::skip]
pub const TETROMINO_J: [[[bool; 4]; 4]; 4] = [
   [[o, X, o, o],
    [o, X, X, X],
    [o, o, o, o],
    [o, o, o, o]],

   [[o, o, X, X],
    [o, o, X, o],
    [o, o, X, o],
    [o, o, o, o]],

   [[o, o, o, o],
    [o, X, X, X],
    [o, o, o, X],
    [o, o, o, o]],

   [[o, o, X, o],
    [o, o, X, o],
    [o, X, X, o],
    [o, o, o, o]],
];

#[rustfmt::skip]
pub const TETROMINO_S: [[[bool; 4]; 4]; 4] = [
   [[o, o, X, X],
    [o, X, X, o],
    [o, o, o, o],
    [o, o, o, o]],

   [[o, o, X, o],
    [o, o, X, X],
    [o, o, o, X],
    [o, o, o, o]],

   [[o, o, o, o],
    [o, o, X, X],
    [o, X, X, o],
    [o, o, o, o]],

   [[o, X, o, o],
    [o, X, X, o],
    [o, o, X, o],
    [o, o, o, o]],
];

#[rustfmt::skip]
pub const TETROMINO_Z: [[[bool; 4]; 4]; 4] = [
   [[o, X, X, o],
    [o, o, X, X],
    [o, o, o, o],
    [o, o, o, o]],

   [[o, o, o, X],
    [o, o, X, X],
    [o, o, X, o],
    [o, o, o, o]],

   [[o, o, o, o],
    [o, X, X, o],
    [o, o, X, X],
    [o, o, o, o]],

   [[o, o, X, o],
    [o, X, X, o],
    [o, X, o, o],
    [o, o, o, o]],
];
