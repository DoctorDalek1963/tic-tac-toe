#![cfg(feature = "bench")]

mod normal {
    use criterion::Criterion;
    use tictactoe::{
        normal::{board::Board, test_utils::make_board},
        CellShape,
    };

    /// Return an assortment of board states to benchmark against.
    fn get_board_states() -> Vec<Board> {
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

mod ultimate {
    use criterion::Criterion;
    use tictactoe::ultimate::{board::GlobalBoard, test_utils::make_global_board};

    const ARRAY_OF_MAX_EXPANSIONS: [u16; 2] = [1000, 5000];
    const ARRAY_OF_PLAYOUTS: [u8; 2] = [1, 5];

    pub mod early_game {
        use super::*;

        fn get_global_board_states() -> Vec<GlobalBoard> {
            vec![
                make_global_board! {
                    next = (0, 2),
                    () (_; _ X _; _) (_; _; X _ _);
                    () (_ O O; _ X _; _) ();
                    () () ();
                },
                make_global_board! {
                    next = (0, 2),
                    () () (_; _; X _ _);
                    () (_ _ O; _ X _; _ _ O) (_; _; _ X _);
                    (_; _ X O; _) (_; _; O _ _) (_; _; X _ _);
                },
                make_global_board! {
                    next = (1, 1),
                    (_; _ _ O; _) () ();
                    (_; _ X _; _ _ X) (_; _ O X; X X X) (_; O X _; _);
                    (_; _ O _; _) (_; _ O _; _) (X _ _; _ O _; _ _ O);
                },
            ]
        }

        pub fn bench_move(c: &mut Criterion) {
            let global_board_states = get_global_board_states();
            let mut group = c.benchmark_group("ultimate::early_game");
            group.sample_size(10);

            for expansions in ARRAY_OF_MAX_EXPANSIONS {
                for playouts in ARRAY_OF_PLAYOUTS {
                    group.bench_function(
                        &format!("generate_ai_move({expansions}, {playouts})"),
                        |b| {
                            b.iter(|| {
                                for global_board in &global_board_states {
                                    global_board.generate_ai_move(expansions, playouts);
                                }
                            })
                        },
                    );
                }
            }
        }
    }

    pub mod late_game {
        use super::*;

        fn get_global_board_states() -> Vec<GlobalBoard> {
            vec![
                make_global_board! {
                    next = (1, 2),
                    (_; O X _; _) (_; O X _; _) (_ X O; _ X O; O O O);
                    (X _ X; X X O; _ O X) (O O O; O X O; O O O) (_ _ X; O X X; _ _ X);
                    (_ _ X; X X _; O _ O) (_ _ X; _ X _; X _ _) (_ _ X; O X O; X X O);
                },
                make_global_board! {
                    next = (1, 2),
                    (_ _ X; _ X _; X O O) (X O X; _ X _; X _ _) (O O O; X X _; _ O _);
                    (_; _ X _; _ _ O) (O O O; O X O; O O O) (_; _ X _; _ X _);
                    (O O _; _ X O; _) (_ _ X; _ X _; X _ O) (_ X X; _ X _; _ X _);
                },
                make_global_board! {
                    next = (0, 2),
                    (X X X; O X X; X X O) (O _ _; X X X; _) (O _ O; X X O; _ _ O);
                    (O O X; X X O; X _ _) (O O O; O X O; O O O) (O O X; O X X; X _ _);
                    (O _ _; O X _; _) (O _ _; _ X _; _) (X _ _; _ X O; _ _ X);
                },
            ]
        }

        pub fn bench_move(c: &mut Criterion) {
            let global_board_states = get_global_board_states();
            let mut group = c.benchmark_group("ultimate::late_game");

            for expansions in ARRAY_OF_MAX_EXPANSIONS {
                for playouts in ARRAY_OF_PLAYOUTS {
                    group.bench_function(
                        &format!("generate_ai_move({expansions}, {playouts})"),
                        |b| {
                            b.iter(|| {
                                for global_board in &global_board_states {
                                    global_board.generate_ai_move(expansions, playouts);
                                }
                            })
                        },
                    );
                }
            }
        }
    }
}

criterion::criterion_group!(
    benches,
    normal::bench_eval_and_move,
    ultimate::early_game::bench_move,
    ultimate::late_game::bench_move,
);
criterion::criterion_main!(benches);
