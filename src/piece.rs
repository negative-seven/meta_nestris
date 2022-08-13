use enum_primitive_derive::Primitive;
use num_traits::ToPrimitive;

#[derive(Clone, Copy, Eq, PartialEq, Primitive)]
pub enum Piece {
    TUp = 0,
    TRight = 1,
    TDown = 2,
    TLeft = 3,
    JUp = 4,
    JRight = 5,
    JDown = 6,
    JLeft = 7,
    ZHorizontal = 8,
    ZVertical = 9,
    O = 10,
    SHorizontal = 11,
    SVertical = 12,
    LUp = 13,
    LRight = 14,
    LDown = 15,
    LLeft = 16,
    IVertical = 17,
    IHorizontal = 18,
    None = 19,
}

impl Piece {
    const TILE_OFFSETS: [[(i8, i8); 4]; 19] = [
        [(-1, 0), (0, 0), (1, 0), (0, -1)],
        [(0, -1), (0, 0), (1, 0), (0, 1)],
        [(-1, 0), (0, 0), (1, 0), (0, 1)],
        [(0, -1), (-1, 0), (0, 0), (0, 1)],
        [(0, -1), (0, 0), (-1, 1), (0, 1)],
        [(-1, -1), (-1, 0), (0, 0), (1, 0)],
        [(0, -1), (1, -1), (0, 0), (0, 1)],
        [(-1, 0), (0, 0), (1, 0), (1, 1)],
        [(-1, 0), (0, 0), (0, 1), (1, 1)],
        [(1, -1), (0, 0), (1, 0), (0, 1)],
        [(-1, 0), (0, 0), (-1, 1), (0, 1)],
        [(0, 0), (1, 0), (-1, 1), (0, 1)],
        [(0, -1), (0, 0), (1, 0), (1, 1)],
        [(0, -1), (0, 0), (0, 1), (1, 1)],
        [(-1, 0), (0, 0), (1, 0), (-1, 1)],
        [(-1, -1), (0, -1), (0, 0), (0, 1)],
        [(1, -1), (-1, 0), (0, 0), (1, 0)],
        [(0, -2), (0, -1), (0, 0), (0, 1)],
        [(-2, 0), (-1, 0), (0, 0), (1, 0)],
    ];
    const PIECE_ROTATIONS: [Piece; 38] = [
        Piece::TLeft,
        Piece::TRight,
        Piece::TUp,
        Piece::TDown,
        Piece::TRight,
        Piece::TLeft,
        Piece::TDown,
        Piece::TUp,
        Piece::JLeft,
        Piece::JRight,
        Piece::JUp,
        Piece::JDown,
        Piece::JRight,
        Piece::JLeft,
        Piece::JDown,
        Piece::JUp,
        Piece::ZVertical,
        Piece::ZVertical,
        Piece::ZHorizontal,
        Piece::ZHorizontal,
        Piece::O,
        Piece::O,
        Piece::SVertical,
        Piece::SVertical,
        Piece::SHorizontal,
        Piece::SHorizontal,
        Piece::LLeft,
        Piece::LRight,
        Piece::LUp,
        Piece::LDown,
        Piece::LRight,
        Piece::LLeft,
        Piece::LDown,
        Piece::LUp,
        Piece::IHorizontal,
        Piece::IHorizontal,
        Piece::IVertical,
        Piece::IVertical,
    ];

    pub fn to_id(self) -> u8 {
        self.to_u8().unwrap()
    }

    pub fn get_counterclockwise_rotation(self) -> Self {
        Self::PIECE_ROTATIONS[self as usize * 2]
    }

    pub fn get_clockwise_rotation(self) -> Self {
        Self::PIECE_ROTATIONS[self as usize * 2 + 1]
    }

    pub fn get_tile_offsets(self) -> &'static [(i8, i8); 4] {
        &Self::TILE_OFFSETS[self as usize]
    }
}
