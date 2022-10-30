use super::{board::GlobalBoard, GlobalCoord};
use crate::shared::CellShape;

pub struct TicTacToeApp {
    global_board: GlobalBoard,
    active_shape: CellShape,
}

impl Default for TicTacToeApp {
    fn default() -> Self {
        Self::new(CellShape::X)
    }
}

impl TicTacToeApp {
    pub fn new(active_shape: CellShape) -> Self {
        Self {
            global_board: GlobalBoard::new(active_shape.other()),
            active_shape,
        }
    }

    fn update_cell(&mut self, coord: GlobalCoord) {
        let (x, y, (lx, ly)) = coord;

        if x > 2 || y > 2 || lx > 2 || ly > 2 {
            return;
        }

        let lb = &mut self.global_board.local_boards[x][y];
        if lb.cells[lx][ly].is_none() {
            lb.cells[lx][ly] = Some(self.active_shape);
            self.active_shape = self.active_shape.other();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ultimate::test_utils::make_board;

    #[test]
    fn update_cell_test() {
        let moves_map: Vec<(GlobalCoord, GlobalBoard)> = vec![
            (
                (1, 1, (1, 1)),
                make_board! {
                    () () ();
                    () (E; E X E; E) ();
                    () () ()
                },
            ),
            (
                (1, 1, (0, 0)),
                make_board! {
                    () () ();
                    () (O E E; E X E; E) ();
                    () () ()
                },
            ),
            (
                (0, 0, (0, 1)),
                make_board! {
                    (E; X E E; E) () ();
                    () (O E E; E X E; E) ();
                    () () ()
                },
            ),
            (
                (0, 1, (1, 1)),
                make_board! {
                    (E; X E E; E) () ();
                    (E; E O E; E) (O E E; E X E; E) ();
                    () () ()
                },
            ),
            (
                (1, 1, (0, 2)),
                make_board! {
                    (E; X E E; E) () ();
                    (E; E O E; E) (O E E; E X E; X E E) ();
                    () () ()
                },
            ),
        ];

        let mut app = TicTacToeApp::default();
        assert_eq!(app.global_board, GlobalBoard::default());

        for (coord, global_board) in moves_map {
            app.update_cell(coord);
            assert_eq!(app.global_board, global_board);
        }
    }
}
