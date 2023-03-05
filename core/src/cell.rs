#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell {
    content: u8,
}

impl Cell {
    const QUEEN: u8 = 1;
    const HORIZONTAL: u8 = 1 << 1;
    const VERTICAL: u8 = 1 << 2;
    const PRINCIPAL: u8 = 1 << 3;
    const ANTIDIAGONAL: u8 = 1 << 4;

    pub const fn new(content: u8) -> Self {
        Self { content }
    }

    pub const fn is_queen(&self) -> bool {
        (self.content & Cell::QUEEN) == Cell::QUEEN
    }

    pub const fn is_attacked(&self) -> bool {
        self.content != 0
    }

    pub const fn is_attacked_horizontal(&self) -> bool {
        (self.content & Cell::HORIZONTAL) == Cell::HORIZONTAL
    }

    pub const fn is_attacked_vertical(&self) -> bool {
        (self.content & Cell::VERTICAL) == Cell::VERTICAL
    }

    pub const fn is_attacked_principal(&self) -> bool {
        (self.content & Cell::PRINCIPAL) == Cell::PRINCIPAL
    }

    pub const fn is_attacked_antidiagonal(&self) -> bool {
        (self.content & Cell::ANTIDIAGONAL) == Cell::ANTIDIAGONAL
    }

    pub const fn is_free(&self) -> bool {
        self.content == 0
    }

    pub fn clear(&mut self) -> &mut Self {
        self.content = 0;
        self
    }

    pub fn put_queen(&mut self) -> &mut Self {
        self.content |= Cell::QUEEN;
        self
    }

    pub fn remove_queen(&mut self) -> &mut Self {
        self.content &= !Cell::QUEEN;
        self
    }

    pub fn attack_horizontal(&mut self) -> &mut Self {
        self.content |= Cell::HORIZONTAL;
        self
    }

    pub fn attack_vertical(&mut self) -> &mut Self {
        self.content |= Cell::VERTICAL;
        self
    }

    pub fn attack_principal(&mut self) -> &mut Self {
        self.content |= Cell::PRINCIPAL;
        self
    }

    pub fn attack_antidiagonal(&mut self) -> &mut Self {
        self.content |= Cell::ANTIDIAGONAL;
        self
    }

    pub fn lift_horizontal(&mut self) -> &mut Self {
        self.content &= !Cell::HORIZONTAL;
        self
    }

    pub fn lift_vertical(&mut self) -> &mut Self {
        self.content &= !Cell::VERTICAL;
        self
    }

    pub fn lift_principal(&mut self) -> &mut Self {
        self.content &= !Cell::PRINCIPAL;
        self
    }

    pub fn lift_antidiagonal(&mut self) -> &mut Self {
        self.content &= !Cell::ANTIDIAGONAL;
        self
    }
}
