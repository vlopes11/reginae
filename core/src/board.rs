use crate::{vec, BTreeSet, Cell, Vec};
use core::mem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    cells: Vec<Cell>,
    queens: BTreeSet<usize>,
    width: usize,
}

impl Board {
    pub fn new(width: usize) -> Self {
        let cells = width * width;
        let cells = vec![Cell::default(); cells];
        let queens = BTreeSet::new();
        Self {
            cells,
            queens,
            width,
        }
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    /// Traverses all the cells attacked by the given index, with the following order: horizontal,
    /// vertical, principal diagonal, antidiagonal.
    ///
    /// # Example
    ///
    /// A board of width 8 will produce the following iterations for the index `0`:
    /// - horizontal: 0..=7
    /// - vertical: (0..=56).step_by(8)
    /// - principal: (0..=63).step_by(9)
    /// - antidiagonal: (0..=0)
    pub fn traverse_boundaries(&self, index: usize) -> impl Iterator<Item = (usize, &Cell)> {
        let bounds = Boundaries::new(index, self.width);
        (bounds.horizontal_min..=bounds.horizontal_max)
            .map(|i| (i, &self.cells[i]))
            .chain(
                (bounds.vertical_min..=bounds.vertical_max)
                    .step_by(self.width)
                    .map(|i| (i, &self.cells[i])),
            )
            .chain(
                (bounds.principal_min..=bounds.principal_max)
                    .step_by(self.width + 1)
                    .map(|i| (i, &self.cells[i])),
            )
            .chain(
                (bounds.antidiagonal_min..=bounds.antidiagonal_max)
                    .step_by(self.width - 1)
                    .map(|i| (i, &self.cells[i])),
            )
    }

    pub fn is_solved(&self) -> bool {
        self.width == self.queens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queens.is_empty()
    }

    pub fn is_queen(&self, index: usize) -> bool {
        self.cells[index].is_queen()
    }

    pub fn rows(&self) -> impl Iterator<Item = &[Cell]> {
        self.cells.chunks(self.width)
    }

    pub fn sorted_queens(&self) -> impl Iterator<Item = usize> + '_ {
        self.queens.iter().copied()
    }

    pub fn toggle_with_pair(&mut self, column: usize, row: usize) -> &mut Self {
        let index = row * self.width + column;
        self.toggle(index)
    }

    pub fn clear(&mut self) -> &mut Self {
        #[cfg(feature = "tracing")]
        tracing::trace!("clearing board");

        self.cells.iter_mut().for_each(|c| {
            c.clear();
        });
        self.queens.clear();
        self
    }

    pub fn take_queens(&mut self) -> BTreeSet<usize> {
        #[cfg(feature = "tracing")]
        tracing::trace!("clearing board");

        self.cells.iter_mut().for_each(|c| {
            c.clear();
        });
        mem::take(&mut self.queens)
    }

    pub fn available(&self) -> impl Iterator<Item = usize> + '_ {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, c)| c.is_free().then_some(i))
    }

    pub fn cells(&self) -> impl Iterator<Item = &'_ Cell> {
        self.cells.iter()
    }

    pub fn toggle(&mut self, index: usize) -> &mut Self {
        if self.cells[index].is_free() {
            self.put_queen(index)
        } else if self.cells[index].is_queen() {
            self.remove_queen(index)
        } else {
            self
        }
    }

    fn put_queen(&mut self, index: usize) -> &mut Self {
        #[cfg(feature = "tracing")]
        tracing::trace!("put queen {index}");

        self.cells[index].put_queen();
        self.queens.insert(index);

        // update the attacked cells
        let bounds = Boundaries::new(index, self.width);
        for i in bounds.horizontal_min..=bounds.horizontal_max {
            self.cells[i].attack_horizontal();
        }
        for i in (bounds.vertical_min..=bounds.vertical_max).step_by(self.width) {
            self.cells[i].attack_vertical();
        }
        for i in (bounds.principal_min..=bounds.principal_max).step_by(self.width + 1) {
            self.cells[i].attack_principal();
        }
        for i in (bounds.antidiagonal_min..=bounds.antidiagonal_max).step_by(self.width - 1) {
            self.cells[i].attack_antidiagonal();
        }

        self
    }

    fn remove_queen(&mut self, index: usize) -> &mut Self {
        #[cfg(feature = "tracing")]
        tracing::trace!("remove queen {index}");

        self.cells[index].remove_queen();
        self.queens.remove(&index);

        // update the attacked cells
        let bounds = Boundaries::new(index, self.width);
        for i in bounds.horizontal_min..=bounds.horizontal_max {
            self.cells[i].lift_horizontal();
        }
        for i in (bounds.vertical_min..=bounds.vertical_max).step_by(self.width) {
            self.cells[i].lift_vertical();
        }
        for i in (bounds.principal_min..=bounds.principal_max).step_by(self.width + 1) {
            self.cells[i].lift_principal();
        }
        for i in (bounds.antidiagonal_min..=bounds.antidiagonal_max).step_by(self.width - 1) {
            self.cells[i].lift_antidiagonal();
        }

        self
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Boundaries {
    pub horizontal_min: usize,
    pub horizontal_max: usize,
    pub vertical_min: usize,
    pub vertical_max: usize,
    pub principal_min: usize,
    pub principal_max: usize,
    pub antidiagonal_min: usize,
    pub antidiagonal_max: usize,
}

impl Boundaries {
    pub fn new(index: usize, width: usize) -> Self {
        let row = index / width;
        let column = index - row * width;
        let min_distance_to_zero = row.min(column);
        let min_column_distance_to_right = row.min(width - column - 1);
        let min_row_distance_to_left = column.min(width - row - 1);
        let min_distance_to_width = (width - row - 1).min(width - column - 1);

        let horizontal_min = row * width;
        let horizontal_max = horizontal_min + width - 1;
        let vertical_min = column;
        let vertical_max = vertical_min + width * (width - 1);
        let principal_min = index - (width + 1) * min_distance_to_zero;
        let principal_max = index + (width + 1) * min_distance_to_width;
        let antidiagonal_min = index - (width - 1) * min_column_distance_to_right;
        let antidiagonal_max = index + (width - 1) * min_row_distance_to_left;

        Self {
            horizontal_min,
            horizontal_max,
            vertical_min,
            vertical_max,
            principal_min,
            principal_max,
            antidiagonal_min,
            antidiagonal_max,
        }
    }
}

#[test]
fn toggle_works() {
    Board::new(8).toggle(0);
}

#[test]
fn boundary_cases() {
    fn case(index: usize, width: usize, boundaries: [usize; 8]) {
        let computed = Boundaries::new(index, width);
        assert_eq!(boundaries[0], computed.horizontal_min);
        assert_eq!(boundaries[1], computed.horizontal_max);
        assert_eq!(boundaries[2], computed.vertical_min);
        assert_eq!(boundaries[3], computed.vertical_max);
        assert_eq!(boundaries[4], computed.principal_min);
        assert_eq!(boundaries[5], computed.principal_max);
        assert_eq!(boundaries[6], computed.antidiagonal_min);
        assert_eq!(boundaries[7], computed.antidiagonal_max);
    }

    case(0, 8, [0, 7, 0, 56, 0, 63, 0, 0]);
    case(7, 8, [0, 7, 7, 63, 7, 7, 7, 56]);
    case(56, 8, [56, 63, 0, 56, 56, 56, 7, 56]);
    case(63, 8, [56, 63, 7, 63, 0, 63, 63, 63]);
    case(8, 8, [8, 15, 0, 56, 8, 62, 1, 8]);
    case(50, 8, [48, 55, 2, 58, 32, 59, 15, 57]);
    case(37, 8, [32, 39, 5, 61, 1, 55, 23, 58]);
    case(0, 9, [0, 8, 0, 72, 0, 80, 0, 0]);
    case(8, 9, [0, 8, 8, 80, 8, 8, 8, 72]);
    case(72, 9, [72, 80, 0, 72, 72, 72, 8, 72]);
    case(80, 9, [72, 80, 8, 80, 0, 80, 80, 80]);
    case(40, 9, [36, 44, 4, 76, 0, 80, 8, 72]);
    case(30, 9, [27, 35, 3, 75, 0, 80, 6, 54]);
    case(31, 9, [27, 35, 4, 76, 1, 71, 7, 63]);
    case(32, 9, [27, 35, 5, 77, 2, 62, 8, 72]);
    case(41, 9, [36, 44, 5, 77, 1, 71, 17, 73]);
    case(50, 9, [45, 53, 5, 77, 0, 80, 26, 74]);
    case(49, 9, [45, 53, 4, 76, 9, 79, 17, 73]);
    case(48, 9, [45, 53, 3, 75, 18, 78, 8, 72]);
    case(39, 9, [36, 44, 3, 75, 9, 79, 7, 63]);
    case(2, 9, [0, 8, 2, 74, 2, 62, 2, 18]);
    case(52, 9, [45, 53, 7, 79, 2, 62, 44, 76]);
}

#[test]
fn traverse_boundaries_works() {
    fn case<Q>(index: usize, width: usize, values: Q)
    where
        Q: IntoIterator<Item = usize>,
    {
        let computed: Vec<_> = Board::new(width)
            .traverse_boundaries(index)
            .map(|(i, _)| i)
            .collect();
        let expected: Vec<_> = values.into_iter().collect();
        assert_eq!(computed, expected);
    }

    case(
        0,
        8,
        (0..8)
            .chain([0, 8, 16, 24, 32, 40, 48, 56].into_iter())
            .chain([0, 9, 18, 27, 36, 45, 54, 63].into_iter())
            .chain([0].into_iter()),
    );
}
