use criterion::{criterion_group, criterion_main};

mod normal {
    use criterion::Criterion;
    use tictactoe::{normal::board::Board, CellShape};

    /// Return an assortment of board states to benchmark against.
    fn get_board_states() -> Vec<Board> {
        // This macro code was copied from src/test_utils.rs and src/normal/test_utils.rs
        #[rustfmt::skip]
        macro_rules! mock_cell_shape {
            (X) => { Some(::tictactoe::CellShape::X) };
            (O) => { Some(::tictactoe::CellShape::O) };
            (_) => { None };
        }

        macro_rules! make_board {
            ($a:tt $b:tt $c:tt; $d:tt $e:tt $f:tt; $g:tt $h:tt $i:tt) => {{
                ::tictactoe::normal::board::Board {
                    cells: [
                        [
                            mock_cell_shape!($a),
                            mock_cell_shape!($d),
                            mock_cell_shape!($g),
                        ],
                        [
                            mock_cell_shape!($b),
                            mock_cell_shape!($e),
                            mock_cell_shape!($h),
                        ],
                        [
                            mock_cell_shape!($c),
                            mock_cell_shape!($f),
                            mock_cell_shape!($i),
                        ],
                    ],
                    ai_shape: ::tictactoe::CellShape::O,
                }
            }};
        }

        vec![
            Board::default(),
            make_board!(_ _ O; _ X _; _ _ X),
            make_board!(_ _ X; _ X O; _ _ _),
            make_board!(O _ O; _ X _; _ X X),
            make_board!(O _ O; _ X _; X _ X),
            make_board!(O X _; _ O X; X _ O),
            make_board!(O X O; X O X; O X X),
            make_board!(X _ O; X O _; _ _ _),
            make_board!(X O _; _ X _; _ _ _),
            make_board!(X O _; _ X O; O _ X),
            make_board!(X O _; X O O; X O _),
            make_board!(X O O; O X X; X X O),
            make_board!(X O X; _ X O; _ O X),
            make_board!(X O X; _ X O; O X O),
            make_board!(X O X; X O _; _ _ _),
            make_board!(X O X; X X O; O _ O),
            make_board!(X X _; _ O _; O _ _),
            make_board!(X X O; O X X; O _ O),
            make_board!(X X X; O O O; _ _ _),
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
