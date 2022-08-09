use enum_primitive_derive::Primitive;
use num_traits::{FromPrimitive, ToPrimitive};

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
}

impl Piece {
    pub fn to_id(&self) -> u8 {
        self.to_u8().unwrap()
    }
}
