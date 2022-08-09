use crate::piece::Piece;
use lazy_static::lazy_static;

#[derive(Clone, Eq, PartialEq)]
pub struct Random {
    pub index: u16,
    pub piece_counter: u8,
    pub last_piece: Piece,
}

impl Random {
    const RNG_STATES_COUNT: u16 = 32767;

    pub fn new() -> Self {
        Self {
            index: 0,
            piece_counter: 0,
            last_piece: Piece::TUp,
        }
    }

    pub fn step(&mut self) {
        self.index += 1;
        self.index %= Self::RNG_STATES_COUNT;
    }

    pub fn choose_random_holes(&mut self) {
        for _ in 0..2 {
            loop {
                self.index += 1;
                self.index %= Self::RNG_STATES_COUNT;

                if (self.get_value() % 16) < 10 {
                    break;
                }
            }
        }
    }

    pub fn next_piece(&mut self) -> Piece {
        const PIECE_TABLE: [Piece; 7] = [
            Piece::TDown,
            Piece::JLeft,
            Piece::ZHorizontal,
            Piece::O,
            Piece::SHorizontal,
            Piece::LRight,
            Piece::IHorizontal,
        ];

        fn get_piece(index: u8) -> Piece {
            PIECE_TABLE[usize::from(index)]
        }

        self.piece_counter = u8::wrapping_add(self.piece_counter, 1);
        let mut piece_index = u8::wrapping_add(self.get_value(), self.piece_counter) % 8;
        if (piece_index as usize) >= PIECE_TABLE.len() || get_piece(piece_index) == self.last_piece {
            self.index += 1;
            piece_index = ((self.get_value() % 8) + self.last_piece.to_id()) % 7;
        }

        self.last_piece = get_piece(piece_index);

        return self.last_piece;
    }

    pub fn get_value(&self) -> u8 {
        lazy_static! {
            static ref RNG_VALUES: [u8; Random::RNG_STATES_COUNT as usize] = {
                let mut values = [0; Random::RNG_STATES_COUNT as usize];

                let mut current: u16 = 0x8988;
                for i in 0..values.len() {
                    values[i] = (current >> 8) as u8;

                    let new_bit = ((current >> 9) ^ (current >> 1)) & 1;
                    current = (new_bit << 15) | (current >> 1);
                }
                return values;
            };
        }

        return RNG_VALUES[usize::from(self.index)];
    }
}
