/// Make a local board with various syntaxes.
///
/// Use `()` for an empty board, or specify a whole board, using `X` or `O` respectively, and `E`
/// for an empty cell.
///
/// # Example
///
/// The call:
/// ```
/// _make_local_board!(X E O; X O E; E E E);
/// ```
/// would look like this:
/// ```text
/// X| |O
/// -----
/// X|O|
/// -----
///  | |
/// ```
macro_rules! _make_local_board {
    (()) => {
        $crate::ultimate::board::LocalBoard::new()
    };
    ((E; E; E)) => {
        $crate::ultimate::board::LocalBoard::new()
    };
    ((E; $d:ident $e:ident $f:ident; $g:ident $h:ident $i:ident)) => {
        $crate::ultimate::board::LocalBoard {
            cells: [
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
            ],
        }
    };
    (($a:ident $b:ident $c:ident; E; $g:ident $h:ident $i:ident)) => {
        $crate::ultimate::board::LocalBoard {
            cells: [
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
            ],
        }
    };
    (($a:ident $b:ident $c:ident; $d:ident $e:ident $f:ident; E)) => {
        $crate::ultimate::board::LocalBoard {
            cells: [
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
            ],
        }
    };
    ((E; E; $g:ident $h:ident $i:ident)) => {
        $crate::ultimate::board::LocalBoard {
            cells: [
                [None, None, $crate::test_utils::mock_cell_shape!($g)],
                [None, None, $crate::test_utils::mock_cell_shape!($h)],
                [None, None, $crate::test_utils::mock_cell_shape!($i)],
            ],
        }
    };
    ((E; $d:ident $e:ident $f:ident; E)) => {
        $crate::ultimate::board::LocalBoard {
            cells: [
                [None, $crate::test_utils::mock_cell_shape!($d), None],
                [None, $crate::test_utils::mock_cell_shape!($e), None],
                [None, $crate::test_utils::mock_cell_shape!($f), None],
            ],
        }
    };
    (($a:ident $b:ident $c:ident; E; E)) => {
        $crate::ultimate::board::LocalBoard {
            cells: [
                [$crate::test_utils::mock_cell_shape!($a), None, None],
                [$crate::test_utils::mock_cell_shape!($b), None, None],
                [$crate::test_utils::mock_cell_shape!($c), None, None],
            ],
        }
    };
    (($a:ident $b:ident $c:ident; $d:ident $e:ident $f:ident; $g:ident $h:ident $i:ident)) => {
        $crate::ultimate::board::LocalBoard {
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
        }
    };
}

/// Make an array of arrays of local boards for use in a global board.
macro_rules! _make_local_board_arrays {
    ($a:tt $b:tt $c:tt; $d:tt $e:tt $f:tt; $g:tt $h:tt $i:tt) => {
        [
            [
                $crate::ultimate::test_utils::_make_local_board!($a),
                $crate::ultimate::test_utils::_make_local_board!($d),
                $crate::ultimate::test_utils::_make_local_board!($g),
            ],
            [
                $crate::ultimate::test_utils::_make_local_board!($b),
                $crate::ultimate::test_utils::_make_local_board!($e),
                $crate::ultimate::test_utils::_make_local_board!($h),
            ],
            [
                $crate::ultimate::test_utils::_make_local_board!($c),
                $crate::ultimate::test_utils::_make_local_board!($f),
                $crate::ultimate::test_utils::_make_local_board!($i),
            ],
        ]
    };
}

/// Make a global board with the given local board macro syntaxes and an optional next local board
/// coordinate specifier at the start, which can be a tuple or `None`.
///
/// See [`_make_local_board`] for the local board syntax.
macro_rules! make_global_board {
    (next = None, $($arr:tt)+ ) => {
        $crate::ultimate::board::GlobalBoard::with_local_boards_and_next_local_board(
            None,
            $crate::ultimate::test_utils::_make_local_board_arrays!($($arr)+),
        )
    };

    (next = ($x:literal, $y:literal), $($arr:tt)+ ) => {
        $crate::ultimate::board::GlobalBoard::with_local_boards_and_next_local_board(
            Some(($x, $y)),
            $crate::ultimate::test_utils::_make_local_board_arrays!($($arr)+),
        )
    };

    ( $($arr:tt)+ ) => {
        $crate::ultimate::board::GlobalBoard::with_local_boards(
            $crate::ultimate::test_utils::_make_local_board_arrays!($($arr)+)
        )
    };
}

pub(crate) use {_make_local_board, _make_local_board_arrays, make_global_board};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        shared::CellShape,
        ultimate::board::{GlobalBoard, LocalBoard},
    };

    #[test]
    fn make_local_board_macro_test() {
        assert_eq!(_make_local_board!(()), LocalBoard::new());
        assert_eq!(_make_local_board!((E; E; E)), LocalBoard::new());

        // X|X|
        // -----
        //  |O|
        // -----
        // O| |
        let board = _make_local_board!((X X E; E O E; O E E));
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
        let board = _make_local_board!((X E O; X O E; E));
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
        let board = _make_local_board!((X X O; O X X; O E O));
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

    #[test]
    fn make_local_board_arrays_macro_test() {
        let arr = _make_local_board_arrays! {
            (X X E; O E O; E E E) () ();
            () () ();
            () () ()
        };
        assert!(arr[0][0].cells[0][0] == Some(CellShape::X));
        assert!(arr[0][0].cells[1][0] == Some(CellShape::X));
        assert!(arr[0][0].cells[0][1] == Some(CellShape::O));
        assert!(arr[0][0].cells[2][1] == Some(CellShape::O));
    }

    #[test]
    fn make_global_board_macro_test() {
        let board = make_global_board! {
            () () ();
            () () ();
            () () ()
        };
        assert_eq!(board, GlobalBoard::default());

        let macro_board = make_global_board! {
            (X X E; O E O; E E E) () ();
            () () ();
            () () ()
        };
        let mut board = GlobalBoard::default();
        board.local_boards[0][0].cells[0][0] = Some(CellShape::X);
        board.local_boards[0][0].cells[1][0] = Some(CellShape::X);
        board.local_boards[0][0].cells[0][1] = Some(CellShape::O);
        board.local_boards[0][0].cells[2][1] = Some(CellShape::O);
        assert_eq!(board, macro_board);

        let macro_board = make_global_board! {
            (E; E; X O E) () ();
            () (X O E; E; O E X) ();
            (X X E; E; O E O) () ()
        };
        let mut board = GlobalBoard::default();
        board.local_boards[0][0].cells[0][2] = Some(CellShape::X);
        board.local_boards[0][0].cells[1][2] = Some(CellShape::O);
        board.local_boards[1][1].cells[0][0] = Some(CellShape::X);
        board.local_boards[1][1].cells[1][0] = Some(CellShape::O);
        board.local_boards[1][1].cells[0][2] = Some(CellShape::O);
        board.local_boards[1][1].cells[2][2] = Some(CellShape::X);
        board.local_boards[0][2].cells[0][0] = Some(CellShape::X);
        board.local_boards[0][2].cells[1][0] = Some(CellShape::X);
        board.local_boards[0][2].cells[0][2] = Some(CellShape::O);
        board.local_boards[0][2].cells[2][2] = Some(CellShape::O);
        assert_eq!(board, macro_board);

        let macro_board = make_global_board! {
            (X X X; E O O; E E O) (E; X E O; E) ();
            () (O X E; E; E) (E; E O E; X E E);
            () () ()
        };
        let mut board = GlobalBoard::default();
        board.local_boards[0][0].cells[0][0] = Some(CellShape::X);
        board.local_boards[0][0].cells[1][0] = Some(CellShape::X);
        board.local_boards[0][0].cells[2][0] = Some(CellShape::X);
        board.local_boards[0][0].cells[1][1] = Some(CellShape::O);
        board.local_boards[0][0].cells[2][1] = Some(CellShape::O);
        board.local_boards[0][0].cells[2][2] = Some(CellShape::O);
        board.local_boards[1][0].cells[0][1] = Some(CellShape::X);
        board.local_boards[1][0].cells[2][1] = Some(CellShape::O);
        board.local_boards[1][1].cells[0][0] = Some(CellShape::O);
        board.local_boards[1][1].cells[1][0] = Some(CellShape::X);
        board.local_boards[2][1].cells[1][1] = Some(CellShape::O);
        board.local_boards[2][1].cells[0][2] = Some(CellShape::X);
        assert_eq!(board, macro_board);

        let macro_board = make_global_board! {
            next = None,
            () () ();
            () () ();
            () () ()
        };
        assert_eq!(GlobalBoard::default(), macro_board);

        let macro_board = make_global_board! {
            next = (1, 1),
            (E; E X E; E) () ();
            () () ();
            () () ()
        };
        let mut board = GlobalBoard::default();
        board
            .make_move((0, 0, (1, 1)), CellShape::X)
            .expect("We should be able to play the move (0, 0, (1, 1))");
        assert_eq!(board, macro_board);
    }
}
