use crate::Board;
use core::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedBoard {
    board: Board,
    rotations: usize,
}

impl NormalizedBoard {
    pub fn merge(self, rhs: Self) -> Self {
        Self {
            board: rhs.board,
            rotations: (self.rotations + rhs.rotations) % 4,
        }
    }

    pub fn normalize(&mut self) -> &mut Self {
        let width = self.board.width();
        if self.board.is_empty() {
            return self;
        }

        let mut distances = [0; 4];
        distances.iter_mut().for_each(|d| {
            // safety: the board isn't empty so we are guaranteed to find a queen
            *d = unsafe {
                PolarScan::new(width)
                    .enumerate()
                    .find_map(|(i, q)| self.board.is_queen(q).then_some(i))
                    .unwrap_unchecked()
            };
            self.rotate_clockwise();
        });

        let rotations = if distances[0] <= distances[1].min(distances[2]).min(distances[3]) {
            0
        } else if distances[1] <= distances[2].min(distances[3]) {
            1
        } else if distances[2] <= distances[3] {
            2
        } else {
            3
        };

        for _ in 0..rotations {
            self.rotate_clockwise();
        }

        self.rotations += rotations;
        self.rotations %= 4;
        self
    }

    pub(crate) fn rotate_clockwise(&mut self) -> &mut Self {
        #[cfg(feature = "tracing")]
        tracing::trace!("rotating");

        // clear the cells
        let queens = self.board.take_queens();

        // rotate each queen and update the board
        let width = self.board.width();
        queens.into_iter().for_each(|q| {
            let truncated = q / width;
            let term = 1 + q - truncated * width;
            let q = width * term - truncated - 1;
            self.board.toggle(q);
        });
        self
    }
}

impl From<Board> for NormalizedBoard {
    fn from(board: Board) -> Self {
        let mut normalized = Self {
            board,
            rotations: 0,
        };
        normalized.normalize();
        normalized
    }
}

impl From<NormalizedBoard> for Board {
    fn from(mut board: NormalizedBoard) -> Self {
        let mut rotations = board.rotations;
        while (rotations % 4) != 0 {
            board.rotate_clockwise();
            rotations += 1;
        }
        board.board
    }
}

impl Deref for NormalizedBoard {
    type Target = Board;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}

impl DerefMut for NormalizedBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.board
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PolarScan {
    width: usize,
    column: usize,
    row: usize,
    max: usize,
}

impl PolarScan {
    pub const fn new(width: usize) -> Self {
        Self {
            width,
            column: 0,
            row: 0,
            max: 0,
        }
    }
}

impl Iterator for PolarScan {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // the iterator is depleted. this is probably a bug as it should be unreachable.
        if self.max >= self.width {
            return None;
        }

        // compute the relative index
        let result = self.row * self.width + self.column;

        // rotate for the next iteration
        if self.column == 0 {
            self.max += 1;
            self.column = self.max;
            self.row = 0;
        } else if self.row < self.max {
            self.row += 1;
        } else {
            self.column -= 1;
        }

        Some(result)
    }
}

#[test]
fn polar_scan_works() {
    let mut polar = PolarScan::new(5);
    assert_eq!(polar.next(), Some(0));
    assert_eq!(polar.next(), Some(1));
    assert_eq!(polar.next(), Some(6));
    assert_eq!(polar.next(), Some(5));
    assert_eq!(polar.next(), Some(2));
    assert_eq!(polar.next(), Some(7));
    assert_eq!(polar.next(), Some(12));
    assert_eq!(polar.next(), Some(11));
    assert_eq!(polar.next(), Some(10));
    assert_eq!(polar.next(), Some(3));
    assert_eq!(polar.next(), Some(8));
    assert_eq!(polar.next(), Some(13));
    assert_eq!(polar.next(), Some(18));
    assert_eq!(polar.next(), Some(17));
    assert_eq!(polar.next(), Some(16));
    assert_eq!(polar.next(), Some(15));
    assert_eq!(polar.next(), Some(4));
    assert_eq!(polar.next(), Some(9));
    assert_eq!(polar.next(), Some(14));
    assert_eq!(polar.next(), Some(19));
    assert_eq!(polar.next(), Some(24));
    assert_eq!(polar.next(), Some(23));
    assert_eq!(polar.next(), Some(22));
    assert_eq!(polar.next(), Some(21));
    assert_eq!(polar.next(), Some(20));
    assert_eq!(polar.next(), None);

    let mut polar = PolarScan::new(8);
    assert_eq!(polar.next(), Some(0));
    assert_eq!(polar.next(), Some(1));
    assert_eq!(polar.next(), Some(9));
    assert_eq!(polar.next(), Some(8));
    assert_eq!(polar.next(), Some(2));
    assert_eq!(polar.next(), Some(10));
    assert_eq!(polar.next(), Some(18));
    assert_eq!(polar.next(), Some(17));
    assert_eq!(polar.next(), Some(16));
    assert_eq!(polar.next(), Some(3));
    assert_eq!(polar.next(), Some(11));
    assert_eq!(polar.next(), Some(19));
    assert_eq!(polar.next(), Some(27));
    assert_eq!(polar.next(), Some(26));
    assert_eq!(polar.next(), Some(25));
    assert_eq!(polar.next(), Some(24));
    assert_eq!(polar.next(), Some(4));
    assert_eq!(polar.next(), Some(12));
    assert_eq!(polar.next(), Some(20));
    assert_eq!(polar.next(), Some(28));
    assert_eq!(polar.next(), Some(36));
    assert_eq!(polar.next(), Some(35));
    assert_eq!(polar.next(), Some(34));
    assert_eq!(polar.next(), Some(33));
    assert_eq!(polar.next(), Some(32));
    assert_eq!(polar.next(), Some(5));
    assert_eq!(polar.next(), Some(13));
    assert_eq!(polar.next(), Some(21));
    assert_eq!(polar.next(), Some(29));
    assert_eq!(polar.next(), Some(37));
    assert_eq!(polar.next(), Some(45));
    assert_eq!(polar.next(), Some(44));
    assert_eq!(polar.next(), Some(43));
    assert_eq!(polar.next(), Some(42));
    assert_eq!(polar.next(), Some(41));
    assert_eq!(polar.next(), Some(40));
    assert_eq!(polar.next(), Some(6));
    assert_eq!(polar.next(), Some(14));
    assert_eq!(polar.next(), Some(22));
    assert_eq!(polar.next(), Some(30));
    assert_eq!(polar.next(), Some(38));
    assert_eq!(polar.next(), Some(46));
    assert_eq!(polar.next(), Some(54));
    assert_eq!(polar.next(), Some(53));
    assert_eq!(polar.next(), Some(52));
    assert_eq!(polar.next(), Some(51));
    assert_eq!(polar.next(), Some(50));
    assert_eq!(polar.next(), Some(49));
    assert_eq!(polar.next(), Some(48));
    assert_eq!(polar.next(), Some(7));
    assert_eq!(polar.next(), Some(15));
    assert_eq!(polar.next(), Some(23));
    assert_eq!(polar.next(), Some(31));
    assert_eq!(polar.next(), Some(39));
    assert_eq!(polar.next(), Some(47));
    assert_eq!(polar.next(), Some(55));
    assert_eq!(polar.next(), Some(63));
    assert_eq!(polar.next(), Some(62));
    assert_eq!(polar.next(), Some(61));
    assert_eq!(polar.next(), Some(60));
    assert_eq!(polar.next(), Some(59));
    assert_eq!(polar.next(), Some(58));
    assert_eq!(polar.next(), Some(57));
    assert_eq!(polar.next(), Some(56));
    assert_eq!(polar.next(), None);
}

#[test]
fn rotate_cases() {
    fn case<Q>(width: usize, queens: Q, output: Q)
    where
        Q: IntoIterator<Item = usize>,
    {
        let board = Board::new(width);
        let board = NormalizedBoard::from(board);
        let queens = queens
            .into_iter()
            .fold(board, |mut board, q| {
                board.toggle(q);
                board
            })
            .rotate_clockwise()
            .queens()
            .collect::<Vec<_>>();
        let output = output.into_iter().collect::<Vec<_>>();
        assert_eq!(queens, output, "failed for width {width}");
    }

    case(
        8,
        [3, 14, 18, 31, 33, 44, 48, 61],
        [1, 11, 21, 31, 34, 40, 54, 60],
    );
    case(8, [27], [28]);
    case(8, [28], [36]);
    case(8, [36], [35]);
    case(8, [35], [27]);
    case(9, [40], [40]);
    case(9, [31], [41]);
    case(9, [41], [49]);
    case(9, [49], [39]);
    case(9, [39], [31]);
}
