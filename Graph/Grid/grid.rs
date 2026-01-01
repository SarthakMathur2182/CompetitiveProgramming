/// # Grid ([source](https://github.com/SarthakMathur2182/CompetitiveProgramming/blob/main/Graph/Grid/grid.rs))
///
/// Some common direction arrays + utils for traversing in a grid.
pub mod grid {
    /// This is made from the perspective of traversing a 2-D array, so the direction and the difference might not make sense.
    pub static DIR_GRID4: [(u8, isize, isize); 4] =
        [(b'D', 1, 0), (b'R', 0, 1), (b'U', -1, 0), (b'L', 0, -1)];

    pub static DIR_GRID8: [(isize, isize); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    pub static DIR_GRID_DIAGONAL: [(isize, isize); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];

    pub static DIR_CHESS_KNIGHT: [(isize, isize); 8] = [
        (1, 2isize),
        (-1, 2),
        (-2, -1),
        (-2, 1),
        (1, -2),
        (-1, -2),
        (2, 1),
        (2, -1),
    ];

    pub fn check_grid_bound(i: isize, j: isize, n: usize, m: usize) -> bool {
        i >= 0 && i < n as isize && j >= 0 && j < m as isize
    }
}
use grid::*;
