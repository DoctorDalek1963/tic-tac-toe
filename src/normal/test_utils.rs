/// Convert a series of identifiers into a board to allow for easy testing.
///
/// This macro goes row-wise and separates rows with semicolons, using `_` for an empty cell.
///
/// # Example
///
/// The call:
/// ```
/// make_board!(X _ O; X O _; _);
/// ```
/// would look like this:
/// ```text
/// X| |O
/// -----
/// X|O|
/// -----
///  | |
/// ```
macro_rules! make_board {
    (_; _; _) => {
        $crate::normal::board::Board::default()
    };
    (_; $d:tt $e:tt $f:tt; $g:tt $h:tt $i:tt) => {{
        $crate::normal::board::Board::with_cell_array([
            [
                None,
                $crate::test_utils::mock_cell_shape!($d),
                $crate::test_utils::mock_cell_shape!($g),
            ],
            [
                None,
                $crate::test_utils::mock_cell_shape!($e),
                $crate::test_utils::mock_cell_shape!($h),
            ],
            [
                None,
                $crate::test_utils::mock_cell_shape!($f),
                $crate::test_utils::mock_cell_shape!($i),
            ],
        ])
    }};
    ($a:tt $b:tt $c:tt; _; $g:tt $h:tt $i:tt) => {{
        $crate::normal::board::Board::with_cell_array([
            [
                $crate::test_utils::mock_cell_shape!($a),
                None,
                $crate::test_utils::mock_cell_shape!($g),
            ],
            [
                $crate::test_utils::mock_cell_shape!($b),
                None,
                $crate::test_utils::mock_cell_shape!($h),
            ],
            [
                $crate::test_utils::mock_cell_shape!($c),
                None,
                $crate::test_utils::mock_cell_shape!($i),
            ],
        ])
    }};
    ($a:tt $b:tt $c:tt; $d:tt $e:tt $f:tt; _) => {{
        $crate::normal::board::Board::with_cell_array([
            [
                $crate::test_utils::mock_cell_shape!($a),
                $crate::test_utils::mock_cell_shape!($d),
                None,
            ],
            [
                $crate::test_utils::mock_cell_shape!($b),
                $crate::test_utils::mock_cell_shape!($e),
                None,
            ],
            [
                $crate::test_utils::mock_cell_shape!($c),
                $crate::test_utils::mock_cell_shape!($f),
                None,
            ],
        ])
    }};
    (_; _; $g:tt $h:tt $i:tt) => {{
        $crate::normal::board::Board::with_cell_array([
            [None, None, $crate::test_utils::mock_cell_shape!($g)],
            [None, None, $crate::test_utils::mock_cell_shape!($h)],
            [None, None, $crate::test_utils::mock_cell_shape!($i)],
        ])
    }};
    ($a:tt $b:tt $c:tt; _; _) => {{
        $crate::normal::board::Board::with_cell_array([
            [$crate::test_utils::mock_cell_shape!($a), None, None],
            [$crate::test_utils::mock_cell_shape!($b), None, None],
            [$crate::test_utils::mock_cell_shape!($c), None, None],
        ])
    }};
    (_; $d:tt $e:tt $f:tt; _) => {{
        $crate::normal::board::Board::with_cell_array([
            [None, $crate::test_utils::mock_cell_shape!($d), None],
            [None, $crate::test_utils::mock_cell_shape!($e), None],
            [None, $crate::test_utils::mock_cell_shape!($f), None],
        ])
    }};
    ($a:tt $b:tt $c:tt; $d:tt $e:tt $f:tt; $g:tt $h:tt $i:tt) => {{
        $crate::normal::board::Board::with_cell_array([
            [
                $crate::test_utils::mock_cell_shape!($a),
                $crate::test_utils::mock_cell_shape!($d),
                $crate::test_utils::mock_cell_shape!($g),
            ],
            [
                $crate::test_utils::mock_cell_shape!($b),
                $crate::test_utils::mock_cell_shape!($e),
                $crate::test_utils::mock_cell_shape!($h),
            ],
            [
                $crate::test_utils::mock_cell_shape!($c),
                $crate::test_utils::mock_cell_shape!($f),
                $crate::test_utils::mock_cell_shape!($i),
            ],
        ])
    }};
}

pub(crate) use make_board;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CellShape;

    #[test]
    fn make_board_macro_test() {
        // X|X|
        // -----
        //  |O|
        // -----
        // O| |
        let board = make_board!(X X _; _ O _; O _ _);
        assert_eq!(board.cells[0][0], Some(CellShape::X));
        assert_eq!(board.cells[1][0], Some(CellShape::X));
        assert_eq!(board.cells[2][0], None);
        assert_eq!(board.cells[0][1], None);
        assert_eq!(board.cells[1][1], Some(CellShape::O));
        assert_eq!(board.cells[2][1], None);
        assert_eq!(board.cells[0][2], Some(CellShape::O));
        assert_eq!(board.cells[1][2], None);
        assert_eq!(board.cells[2][2], None);

        // X| |O
        // -----
        // X|O|
        // -----
        //  | |
        let board = make_board!(X _ O; X O _; _);
        assert_eq!(board.cells[0][0], Some(CellShape::X));
        assert_eq!(board.cells[1][0], None);
        assert_eq!(board.cells[2][0], Some(CellShape::O));
        assert_eq!(board.cells[0][1], Some(CellShape::X));
        assert_eq!(board.cells[1][1], Some(CellShape::O));
        assert_eq!(board.cells[2][1], None);
        assert_eq!(board.cells[0][2], None);
        assert_eq!(board.cells[1][2], None);
        assert_eq!(board.cells[2][2], None);

        // X|X|O
        // -----
        // O|X|X
        // -----
        // O| |O
        let board = make_board!(X X O; O X X; O _ O);
        assert_eq!(board.cells[0][0], Some(CellShape::X));
        assert_eq!(board.cells[1][0], Some(CellShape::X));
        assert_eq!(board.cells[2][0], Some(CellShape::O));
        assert_eq!(board.cells[0][1], Some(CellShape::O));
        assert_eq!(board.cells[1][1], Some(CellShape::X));
        assert_eq!(board.cells[2][1], Some(CellShape::X));
        assert_eq!(board.cells[0][2], Some(CellShape::O));
        assert_eq!(board.cells[1][2], None);
        assert_eq!(board.cells[2][2], Some(CellShape::O));
    }
}
