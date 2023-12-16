use core::mem::transmute;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn as_index(self) -> usize {
        self as usize
    }

    pub fn from_u8(val: u8) -> Option<Self> {
        Some(match val {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            3 => Self::Left,
            _ => return None,
        })
    }

    pub fn from_pos(pos: (i32, i32)) -> Option<Self> {
        Some(match pos {
            (0, -1) => Self::Up,
            (1, 0) => Self::Right,
            (0, 1) => Self::Down,
            (-1, 0) => Self::Left,
            _ => return None,
        })
    }

    pub fn move_pos(self, pos: &mut (i32, i32)) {
        let (x, y) = self.as_pos();
        pos.0 += x;
        pos.1 += y;
    }

    pub fn as_pos(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }

    pub fn rotated(self, dir: Self) -> Self {
        let o = (self as u8 + dir as u8) % 4;
        // SAFETY: o is unsigned and a result of a modulo 4 calculation, so always < 4
        unsafe { transmute::<u8, Self>(o) }
    }

    pub fn flipped(self) -> Self {
        self.rotated(Direction::Down)
    }

    pub fn is_horizontal(self) -> bool {
        matches!(self, Self::Right | Self::Left)
    }

    pub fn is_vertical(self) -> bool {
        !self.is_horizontal()
    }
}
