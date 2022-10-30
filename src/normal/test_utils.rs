/// Convert a series of identifiers into a board to allow for easy testing.
///
/// This macro goes row-wise and separates rows with semicolons, using `E` for an empty cell.
///
/// # Example
///
/// The call:
/// ```
/// make_board!(X E O; X O E; E E E);
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
    ($a:ident $b:ident $c:ident; $d:ident $e:ident $f:ident; $g:ident $h:ident $i:ident) => {{
        $crate::normal::board::Board {
            cells: [
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
            ],
            ai_shape: $crate::shared::CellShape::O,
        }
    }};
}

pub(crate) use make_board;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::CellShape;

    #[test]
    fn make_board_macro_test() {
        // X|X|
        // -----
        //  |O|
        // -----
        // O| |
        let board = make_board!(X X E; E O E; O E E);
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
        let board = make_board!(X E O; X O E; E E E);
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
        let board = make_board!(X X O; O X X; O E O);
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
