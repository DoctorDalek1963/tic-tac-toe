use criterion::{criterion_group, criterion_main};

mod normal {
    use criterion::Criterion;
    use tictactoe::{normal::board::Board, CellShape};

    /// Return an assortment of board states to benchmark against.
    fn get_board_states() -> Vec<Board> {
        // This macro code was copied from src/normal/test_utils.rs
        enum MockCellShape {
            X,
            O,
            E,
        }

        impl Into<Option<CellShape>> for MockCellShape {
            fn into(self) -> Option<CellShape> {
                match self {
                    MockCellShape::X => Some(CellShape::X),
                    MockCellShape::O => Some(CellShape::O),
                    MockCellShape::E => None,
                }
            }
        }

        macro_rules! make_board {
            ($a:ident $b:ident $c:ident; $d:ident $e:ident $f:ident; $g:ident $h:ident $i:ident) => {{
                ::tictactoe::normal::board::Board {
                    cells: [
                        [
                            MockCellShape::$a.into(),
                            MockCellShape::$d.into(),
                            MockCellShape::$g.into(),
                        ],
                        [
                            MockCellShape::$b.into(),
                            MockCellShape::$e.into(),
                            MockCellShape::$h.into(),
                        ],
                        [
                            MockCellShape::$c.into(),
                            MockCellShape::$f.into(),
                            MockCellShape::$i.into(),
                        ],
                    ],
                    ai_shape: ::tictactoe::CellShape::O,
                }
            }};
        }

        vec![
            Board::default(),
            make_board!(E E O; E X E; E E X),
            make_board!(E E X; E X O; E E E),
            make_board!(O E O; E X E; E X X),
            make_board!(O E O; E X E; X E X),
            make_board!(O X E; E O X; X E O),
            make_board!(O X O; X O X; O X X),
            make_board!(X E O; X O E; E E E),
            make_board!(X O E; E X E; E E E),
            make_board!(X O E; E X O; O E X),
            make_board!(X O E; X O O; X O E),
            make_board!(X O O; O X X; X X O),
            make_board!(X O X; E X O; E O X),
            make_board!(X O X; E X O; O X O),
            make_board!(X O X; X O E; E E E),
            make_board!(X O X; X X O; O E O),
            make_board!(X X E; E O E; O E E),
            make_board!(X X O; O X X; O E O),
            make_board!(X X X; O O O; E E E),
        ]
    }

    pub fn bench_eval_and_move(c: &mut Criterion) {
        let board_states = get_board_states();

        c.bench_function("normal::evaluate_position", |b| {
            b.iter(|| {
                for board in &board_states {
                    board.evaluate_position(CellShape::X);
                }
            });

            b.iter(|| {
                for board in &board_states {
                    board.evaluate_position(CellShape::O);
                }
            });
        });

        c.bench_function("normal::generate_ai_move", |b| {
            b.iter(|| {
                for board in &board_states {
                    let _ = board.generate_ai_move();
                }
            });
        });
    }
}

criterion_group!(benches, normal::bench_eval_and_move);
criterion_main!(benches);
