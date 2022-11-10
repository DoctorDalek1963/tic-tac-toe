//! This module provides functionality for an AI based on Monte Carlo tree search (MCTS).

use super::GlobalBoard;
use crate::{shared::board::WinnerError, ultimate::GlobalCoord, CellShape};
use rand::{seq::SliceRandom, thread_rng};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// A struct to represent a node in a game tree.
#[derive(Clone, Debug)]
struct Node {
    /// The move taken to get to this board state.
    previous_move: Option<GlobalCoord>,

    /// The board state of this node.
    board: RefCell<GlobalBoard>,

    /// The shape to play next.
    shape_to_play_next: CellShape,

    /// The total `(wins, playouts)` of this node.
    ///
    /// The wins *always* refer to wins for the AI, as opposed to a win for the player about to
    /// play in this position.
    wins_vs_playouts: RefCell<(u16, u16)>,

    /// The parent node in the game tree.
    parent: Weak<Node>,

    /// A vector of child nodes.
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    /// Create a root node with no parent or children, and the given data.
    fn make_root(board: &GlobalBoard, shape_to_play_next: CellShape) -> Self {
        Self {
            previous_move: None,
            board: RefCell::new(board.clone()),
            shape_to_play_next,
            wins_vs_playouts: RefCell::new((0, 0)),
            parent: Weak::new(),
            children: RefCell::new(vec![]),
        }
    }

    /// Check if this node is a leaf.
    fn is_leaf(&self) -> bool {
        self.children.borrow().len() == 0
    }

    /// Compute the [UCT](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search#Exploration_and_exploitation) of this node.
    ///
    /// The UCT is a number assigned to a node to assess how much we want to explore it. It's meant
    /// to balance exploitation of promising nodes with exploration of new nodes.
    ///
    /// The UCT is calculated with the following formula:
    /// ```text
    /// (w / n) + c * sqrt(ln(N) / n)
    /// ```
    /// where `w` is the number of wins of this node, `n` is the total number of playouts of this
    /// node, `N` is the total number of playouts of the parent node, and `c` is the "exploration
    /// parameter".
    ///
    /// This method returns an `Option<f64>` because if this node doesn't have a parent, then we
    /// can't calculate `N`, so we can't calculate a UCT. The root node of the game tree doesn't
    /// have a UCT because it's the root. We don't have any other sibling nodes to choose to
    /// explore instead.
    fn compute_uct(&self) -> Option<f64> {
        const EXPLORATION_PARAMETER: f64 = 1.414;

        let wins: f64 = self.wins_vs_playouts.borrow().0 as f64;
        let total_playouts: f64 = self.wins_vs_playouts.borrow().1 as f64;
        let parent_total_playouts: f64 = self.parent.upgrade()?.wins_vs_playouts.borrow().1 as f64;

        let uct = (wins / total_playouts)
            + EXPLORATION_PARAMETER * f64::sqrt(f64::ln(parent_total_playouts) / total_playouts);

        Some(uct)
    }

    /// Find the best child of the given node by comparing UCT values.
    fn best_child_by_uct(node: &Rc<Node>) -> Option<Rc<Node>> {
        const MSG: &'static str =
            "We should never try to compute the UCT of the root node, so it should never be none";

        let children = node.children.borrow();
        children
            .iter()
            .max_by(|&c1, &c2| {
                c1.compute_uct()
                    .expect(MSG)
                    .total_cmp(&c2.compute_uct().expect(MSG))
            })
            .map(Rc::clone)
    }

    /// Select the next node to expand and return an [`Rc`] to it.
    ///
    /// This function should only be manually called on the root node, and then it will traverse
    /// down the children to find the best leaf.
    fn select_node(node: &Rc<Node>) -> Rc<Node> {
        if node.is_leaf() {
            return Rc::clone(node);
        }

        let best_child = Node::best_child_by_uct(node)
            .expect("We know this node has children, or else we would've returned self earlier");
        Node::select_node(&best_child)
    }

    /// Expand the current node if possible.
    ///
    /// If this board state results in a win, loss, or draw, or if there are no legal moves, then
    /// no expansion will happen and no children will be created. Otherwise, we will create a child
    /// node for each legal move.
    fn expand(node: &Rc<Node>) {
        let legal_moves = node.board.borrow().legal_moves();
        if node.board.borrow_mut().get_winner() != Err(WinnerError::NoWinnerYet)
            || legal_moves.is_empty()
        {
            return;
        }

        let mut children = node.children.borrow_mut();

        for mv in legal_moves {
            let mut board: GlobalBoard = node.board.borrow().clone();
            board
                .make_move(mv, node.shape_to_play_next)
                .expect("A legal move should never cause a `MoveError`");

            let node = Node {
                previous_move: Some(mv),
                board: RefCell::new(board),
                shape_to_play_next: node.shape_to_play_next.other(),
                wins_vs_playouts: RefCell::new((0, 0)),
                parent: Rc::downgrade(node),
                children: RefCell::new(vec![]),
            };

            node.playout_and_backpropagate();
            children.push(Rc::new(node));
        }
    }

    /// Play the game to completion with random moves and return whether the AI won this simulation.
    fn playout(&self) -> bool {
        let mut board: GlobalBoard = self.board.borrow().clone();
        let mut shape = self.shape_to_play_next;

        // Keep making moves until either someone wins, or there's a draw
        while let Err(WinnerError::NoWinnerYet) = board.get_winner() {
            let Some(coord) = board.get_random_legal_move() else {
                break;
            };

            board
                .make_move(coord, shape)
                .expect("When making a random legal move, we should not get a `MoveError`");
            shape = shape.other();
        }

        match board.get_winner() {
            Ok((winning_shape, _)) if winning_shape == self.board.borrow().ai_shape => true,
            _ => false,
        }
    }

    /// Propagate a win or loss up the game tree to the root node.
    fn backpropagate(&self, has_won: bool) {
        let mut tup = self.wins_vs_playouts.borrow_mut();
        if has_won {
            tup.0 += 1;
        }
        tup.1 += 1;
        drop(tup);

        match self.parent.upgrade() {
            None => (),
            Some(parent) => parent.backpropagate(has_won),
        };
    }

    /// Playout this board state and backpropagate the result up to this node's parents.
    ///
    /// See [`playout`](Self::playout) and [`backpropagate`](Self::backpropagate).
    fn playout_and_backpropagate(&self) {
        self.backpropagate(self.playout());
    }
}

/// The coordinates of all the cells in the global board.
#[rustfmt::skip]
const ALL_CELLS: [GlobalCoord; 81] = [
    (0, 0, (0, 0)), (0, 0, (1, 0)), (0, 0, (2, 0)),
    (0, 0, (0, 1)), (0, 0, (1, 1)), (0, 0, (2, 1)),
    (0, 0, (0, 2)), (0, 0, (1, 2)), (0, 0, (2, 2)),

    (1, 0, (0, 0)), (1, 0, (1, 0)), (1, 0, (2, 0)),
    (1, 0, (0, 1)), (1, 0, (1, 1)), (1, 0, (2, 1)),
    (1, 0, (0, 2)), (1, 0, (1, 2)), (1, 0, (2, 2)),

    (2, 0, (0, 0)), (2, 0, (1, 0)), (2, 0, (2, 0)),
    (2, 0, (0, 1)), (2, 0, (1, 1)), (2, 0, (2, 1)),
    (2, 0, (0, 2)), (2, 0, (1, 2)), (2, 0, (2, 2)),

    (0, 1, (0, 0)), (0, 1, (1, 0)), (0, 1, (2, 0)),
    (0, 1, (0, 1)), (0, 1, (1, 1)), (0, 1, (2, 1)),
    (0, 1, (0, 2)), (0, 1, (1, 2)), (0, 1, (2, 2)),

    (1, 1, (0, 0)), (1, 1, (1, 0)), (1, 1, (2, 0)),
    (1, 1, (0, 1)), (1, 1, (1, 1)), (1, 1, (2, 1)),
    (1, 1, (0, 2)), (1, 1, (1, 2)), (1, 1, (2, 2)),

    (2, 1, (0, 0)), (2, 1, (1, 0)), (2, 1, (2, 0)),
    (2, 1, (0, 1)), (2, 1, (1, 1)), (2, 1, (2, 1)),
    (2, 1, (0, 2)), (2, 1, (1, 2)), (2, 1, (2, 2)),

    (0, 2, (0, 0)), (0, 2, (1, 0)), (0, 2, (2, 0)),
    (0, 2, (0, 1)), (0, 2, (1, 1)), (0, 2, (2, 1)),
    (0, 2, (0, 2)), (0, 2, (1, 2)), (0, 2, (2, 2)),

    (1, 2, (0, 0)), (1, 2, (1, 0)), (1, 2, (2, 0)),
    (1, 2, (0, 1)), (1, 2, (1, 1)), (1, 2, (2, 1)),
    (1, 2, (0, 2)), (1, 2, (1, 2)), (1, 2, (2, 2)),

    (2, 2, (0, 0)), (2, 2, (1, 0)), (2, 2, (2, 0)),
    (2, 2, (0, 1)), (2, 2, (1, 1)), (2, 2, (2, 1)),
    (2, 2, (0, 2)), (2, 2, (1, 2)), (2, 2, (2, 2)),
];

impl GlobalBoard {
    /// Return a vec of all the legal moves on the global board.
    fn legal_moves(&self) -> Vec<GlobalCoord> {
        match self.next_local_board() {
            None => ALL_CELLS.to_vec(),
            #[rustfmt::skip]
            Some((x, y)) => vec![
                (x, y, (0, 0)), (x, y, (1, 0)), (x, y, (2, 0)),
                (x, y, (0, 1)), (x, y, (1, 1)), (x, y, (2, 1)),
                (x, y, (0, 2)), (x, y, (1, 2)), (x, y, (2, 2)),
            ],
        }
        .iter()
        .filter_map(|&(x, y, (lx, ly))| {
            if self.local_boards[x][y].cells[lx][ly].is_none() {
                Some((x, y, (lx, ly)))
            } else {
                None
            }
        })
        .collect()
    }

    /// Return a random legal move, or [`None`] if the board is full.
    fn get_random_legal_move(&self) -> Option<GlobalCoord> {
        self.legal_moves().choose(&mut thread_rng()).copied()
    }

    fn do_mcts(&self, iterations: u16) -> Option<GlobalCoord> {
        if self.legal_moves().is_empty() {
            return None;
        }

        let root = &Rc::new(Node::make_root(self, self.ai_shape));
        Node::expand(root);
        let mut next = Node::select_node(root);

        for _ in 1..iterations {
            Node::expand(&next);
            next = Node::select_node(root);
        }

        let children = root.children.borrow();
        children
            .iter()
            .max_by_key(|&child| child.wins_vs_playouts.borrow().1)?
            .previous_move
    }

    /// Return the AI-chosen optimal move, which could be none if the board is full.
    pub fn generate_ai_move(&self, max_mcts_iterations: u16) -> Option<GlobalCoord> {
        for mv in self.legal_moves() {
            let mut board = self.clone();
            board
                .make_move(mv, self.ai_shape)
                .expect("A legal move should never result in a `MoveError`");

            if matches!(board.get_winner(), Ok((shape, _)) if shape == self.ai_shape) {
                return Some(mv);
            }
        }

        self.do_mcts(max_mcts_iterations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ultimate::test_utils::make_global_board;

    #[test]
    fn legal_moves_test() {
        let board = GlobalBoard::default();
        assert_eq!(board.legal_moves(), ALL_CELLS);

        let board = make_global_board! {
            next = None,
            (_; _ X _; _) (_; _ X _; _) (_; _ X _; _);
            (_; _ X _; _) (O O O; O X O; O O O) (_; _ X _; _);
            (_; _ X _; _) (_; _ X _; _) (_; _ X _; _);
        };
        #[rustfmt::skip]
        assert_eq!(
            board.legal_moves(),
            vec![
                (0, 0, (0, 0)), (0, 0, (1, 0)), (0, 0, (2, 0)),
                (0, 0, (0, 1)), (0, 0, (2, 1)),
                (0, 0, (0, 2)), (0, 0, (1, 2)), (0, 0, (2, 2)),

                (1, 0, (0, 0)), (1, 0, (1, 0)), (1, 0, (2, 0)),
                (1, 0, (0, 1)), (1, 0, (2, 1)),
                (1, 0, (0, 2)), (1, 0, (1, 2)), (1, 0, (2, 2)),

                (2, 0, (0, 0)), (2, 0, (1, 0)), (2, 0, (2, 0)),
                (2, 0, (0, 1)), (2, 0, (2, 1)),
                (2, 0, (0, 2)), (2, 0, (1, 2)), (2, 0, (2, 2)),

                (0, 1, (0, 0)), (0, 1, (1, 0)), (0, 1, (2, 0)),
                (0, 1, (0, 1)), (0, 1, (2, 1)),
                (0, 1, (0, 2)), (0, 1, (1, 2)), (0, 1, (2, 2)),

                (2, 1, (0, 0)), (2, 1, (1, 0)), (2, 1, (2, 0)),
                (2, 1, (0, 1)), (2, 1, (2, 1)),
                (2, 1, (0, 2)), (2, 1, (1, 2)), (2, 1, (2, 2)),

                (0, 2, (0, 0)), (0, 2, (1, 0)), (0, 2, (2, 0)),
                (0, 2, (0, 1)), (0, 2, (2, 1)),
                (0, 2, (0, 2)), (0, 2, (1, 2)), (0, 2, (2, 2)),

                (1, 2, (0, 0)), (1, 2, (1, 0)), (1, 2, (2, 0)),
                (1, 2, (0, 1)), (1, 2, (2, 1)),
                (1, 2, (0, 2)), (1, 2, (1, 2)), (1, 2, (2, 2)),

                (2, 2, (0, 0)), (2, 2, (1, 0)), (2, 2, (2, 0)),
                (2, 2, (0, 1)), (2, 2, (2, 1)),
                (2, 2, (0, 2)), (2, 2, (1, 2)), (2, 2, (2, 2)),
            ]
        );

        let board = make_global_board! {
            next = (1, 1),
            () () ();
            () (_; _ X _; _) ();
            () () ();
        };
        #[rustfmt::skip]
        assert_eq!(
            board.legal_moves(),
            vec![
                (1, 1, (0, 0)), (1, 1, (1, 0)), (1, 1, (2, 0)),
                (1, 1, (0, 1)), (1, 1, (2, 1)),
                (1, 1, (0, 2)), (1, 1, (1, 2)), (1, 1, (2, 2)),
            ]
        );

        let board = make_global_board! {
            next = (0, 0),
            () () ();
            (_; _ X _; O O _) (_; O X X; O _ _) (_; _ O _; _);
            (O _ _; X _ _; X _ _) (_; X _ _; _) ();
        };
        #[rustfmt::skip]
        assert_eq!(
            board.legal_moves(),
            vec![
                (0, 0, (0, 0)), (0, 0, (1, 0)), (0, 0, (2, 0)),
                (0, 0, (0, 1)), (0, 0, (1, 1)), (0, 0, (2, 1)),
                (0, 0, (0, 2)), (0, 0, (1, 2)), (0, 0, (2, 2)),
            ]
        );

        let board = make_global_board! {
            next = (1, 1),
            (_; _ X _; _) () ();
            (_; _ X _; O O _) (_; O X X; O _ _) (_; _ O _; _);
            (O _ _; X _ _; X _ _) (_; X _ _; _) ();
        };
        assert_eq!(
            board.legal_moves(),
            vec![
                (1, 1, (0, 0)),
                (1, 1, (1, 0)),
                (1, 1, (2, 0)),
                (1, 1, (1, 2)),
                (1, 1, (2, 2)),
            ]
        );
    }

    mod tree {
        use super::*;

        fn get_test_root_node() -> Rc<Node> {
            let board = make_global_board! {
                next = (1, 1),
                (_; _; _ _ X) () ();
                () (O _ _; _ X _; _) ();
                () () (_; _ O _; _);
            };
            Rc::new(Node::make_root(&board, CellShape::O))
        }

        #[test]
        fn expand_test() {
            let node_rc = get_test_root_node();
            Node::expand(&node_rc);

            // 7 possible moves
            assert_eq!(node_rc.children.borrow().len(), 7);

            for child in node_rc.children.borrow().iter() {
                // 1 playout
                assert_eq!(child.wins_vs_playouts.borrow().1, 1);
                // <= 1 win(s)
                assert!(child.wins_vs_playouts.borrow().0 <= 1);
            }
        }

        #[test]
        fn backpropagate_test() {
            let node_rc = get_test_root_node();
            Node::expand(&node_rc);

            // 7 playouts
            assert_eq!(node_rc.wins_vs_playouts.borrow().1, 7);
            // <= 7 wins
            assert!(node_rc.wins_vs_playouts.borrow().0 <= 7);
        }
    }
}
