use crate::{Board, Evaluator, NormalizedBoard};
use radix_trie::Trie;

#[derive(Default, Clone)]
pub struct Solver {
    depleted: Trie<Vec<usize>, ()>,
    evaluator: Evaluator,
    jumps: usize,
}

impl Solver {
    pub fn with_evaluator(&mut self, f: fn(&Board, usize) -> f64, weight: f64) -> &mut Self {
        self.evaluator.inject_evaluator(f, weight);
        self
    }

    pub fn solve(&mut self, board: Board) -> Solution {
        let mut normalized = NormalizedBoard::from(board);
        let mut path = Vec::with_capacity(normalized.width());
        let (success, jumps) = self._solve(&mut normalized, &mut path);
        let board = Board::from(normalized);
        Solution {
            board,
            success,
            jumps,
        }
    }

    fn _solve(&mut self, board: &mut NormalizedBoard, path: &mut Vec<usize>) -> (bool, usize) {
        if board.is_empty() {
            board.toggle(0);
        } else if board.is_solved() {
            return (true, self.jumps);
        }

        // check if the path is depleted
        let mut sorted = path.clone();
        sorted.sort();
        if self.depleted.get(&sorted).is_some() {
            return (false, self.jumps);
        }

        self.jumps += 1;

        // build the unexplored nodes list and score them
        let last_move = path.last().copied().unwrap_or(0);
        let mut unexplored: Vec<_> = board
            .available()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|index| {
                board.toggle(index);
                let score = self.evaluator.score(board, last_move);
                board.toggle(index);
                Frontier {
                    depleted: false,
                    index,
                    score,
                }
            })
            .collect();

        // sort by score so we can pop the highest one
        unexplored.sort_by_key(|f| f.score);

        // A* the path recursively
        while let Some(frontier) = unexplored.pop() {
            path.push(frontier.index);
            board.toggle(frontier.index);

            let solution = self._solve(board, path);
            if solution.0 {
                return solution;
            }
            path.pop();
            board.toggle(frontier.index);
        }

        for _ in 0..4 {
            board.rotate_clockwise();
            self.depleted.insert(board.sorted_queens().collect(), ());
        }

        (false, self.jumps)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solution {
    pub board: Board,
    pub success: bool,
    pub jumps: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Frontier {
    depleted: bool,
    index: usize,
    score: u64,
}
