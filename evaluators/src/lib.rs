#![no_std]

use reginae_core::Board;

/// score hight as the attacked cells from the last move produces more overlapped attacks on
/// the same cell (naturally, from other queens)
#[no_mangle]
pub fn overlapping(board: &Board, last_move: usize) -> f64 {
    let width = board.width();
    let mut count = 0_u64;
    let mut boundaries = board.traverse_boundaries(last_move);

    let horizontal: u64 = boundaries
        .by_ref()
        .take(width)
        .map(|(_, c)| {
            count += 1;
            c.is_attacked_vertical() as u64
                + c.is_attacked_principal() as u64
                + c.is_attacked_antidiagonal() as u64
        })
        .sum();

    let vertical: u64 = boundaries
        .by_ref()
        .take(width)
        .map(|(_, c)| {
            count += 1;
            c.is_attacked_horizontal() as u64
                + c.is_attacked_principal() as u64
                + c.is_attacked_antidiagonal() as u64
        })
        .sum();

    let mut is_principal = true;
    let mut last_diagonal = 0;
    let diagonal: u64 = boundaries
        .by_ref()
        .take(width)
        .map(|(i, c)| {
            count += 1;
            if i < last_diagonal {
                is_principal = false;
            }
            last_diagonal = i;
            c.is_attacked_horizontal() as u64
                + c.is_attacked_vertical() as u64
                + if is_principal {
                    c.is_attacked_antidiagonal() as u64
                } else {
                    c.is_attacked_principal() as u64
                }
        })
        .sum();

    let max = count * 3;
    let sum = horizontal + vertical + diagonal;

    sum as f64 / max as f64
}

/// score higher as more queens are ladder to last move (i.e. knight move).
///
/// ladder seems to perform well for odd width, but will cause harm to even width search.
#[no_mangle]
pub fn ladder(board: &Board, last_move: usize) -> f64 {
    let width = board.width();
    let row = last_move / width;
    let column = last_move - row * width;
    let mut count = 0;

    if let Some((column, row)) = column.checked_sub(2).zip(row.checked_sub(1)) {
        let index = row * width + column;
        count += board.is_queen(index) as u64;
    }

    if let Some((column, row)) = column.checked_sub(1).zip(row.checked_sub(2)) {
        let index = row * width + column;
        count += board.is_queen(index) as u64;
    }

    if let Some((column, row)) = Some(column + 1)
        .filter(|c| c < &width)
        .zip(row.checked_sub(2))
    {
        let index = row * width + column;
        count += board.is_queen(index) as u64;
    }

    if let Some((column, row)) = Some(column + 2)
        .filter(|c| c < &width)
        .zip(row.checked_sub(1))
    {
        let index = row * width + column;
        count += board.is_queen(index) as u64;
    }

    if let Some((column, row)) = Some(column + 2)
        .filter(|c| c < &width)
        .zip(Some(row + 1).filter(|c| c < &width))
    {
        let index = row * width + column;
        count += board.is_queen(index) as u64;
    }

    if let Some((column, row)) = Some(column + 1)
        .filter(|c| c < &width)
        .zip(Some(row + 2).filter(|c| c < &width))
    {
        let index = row * width + column;
        count += board.is_queen(index) as u64;
    }

    if let Some((column, row)) = column
        .checked_sub(1)
        .zip(Some(row + 2).filter(|c| c < &width))
    {
        let index = row * width + column;
        count += board.is_queen(index) as u64;
    }

    if let Some((column, row)) = column
        .checked_sub(2)
        .zip(Some(row + 1).filter(|c| c < &width))
    {
        let index = row * width + column;
        count += board.is_queen(index) as u64;
    }

    count as f64 / 8.0
}

/// score higher as more queens are ladder to last move (i.e. knight move), wrapping around the
/// board to produce a toroidal surface.
///
/// can be used in combination with the regular ladder for even width with a negative weight.
#[no_mangle]
pub fn wrapping_ladder(board: &Board, last_move: usize) -> f64 {
    let width = board.width();
    let cells = board.width() * board.width();
    let mut count = 0;

    let x = 2 * width - 1;
    let x = last_move.wrapping_sub(x) % cells;
    count += board.is_queen(x) as u32;

    let x = width - 2;
    let x = last_move.wrapping_sub(x) % cells;
    count += board.is_queen(x) as u32;

    let x = 2 * width + 1;
    let x = last_move.wrapping_sub(x) % cells;
    count += board.is_queen(x) as u32;

    let x = width + 2;
    let x = last_move.wrapping_sub(x) % cells;
    count += board.is_queen(x) as u32;

    let x = width - 2;
    let x = (last_move + x) % cells;
    count += board.is_queen(x) as u32;

    let x = 2 * width - 1;
    let x = (last_move + x) % cells;
    count += board.is_queen(x) as u32;

    let x = 2 * width + 1;
    let x = (last_move + x) % cells;
    count += board.is_queen(x) as u32;

    let x = width + 2;
    let x = (last_move + x) % cells;
    count += board.is_queen(x) as u32;

    count as f64 / 8.0
}
