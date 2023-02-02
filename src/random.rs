use once_cell::sync::Lazy;

use crate::piece::Piece;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Random {
    pub index: u16,
    pub piece_counter: u8,
    pub last_piece: Piece,
}

impl Random {
    const RNG_STATES_COUNT: u16 = 32767;

    #[must_use]
    pub fn new() -> Self {
        Self {
            index: 0,
            piece_counter: 0,
            last_piece: Piece::TUp,
        }
    }

    pub fn cycle(&mut self) {
        self.index += 1;
        self.index %= Self::RNG_STATES_COUNT;
    }

    pub fn cycle_multiple(&mut self, count: usize) {
        for _ in 0..count {
            self.cycle();
        }
    }

    pub fn cycle_do_while(&mut self, condition: impl Fn(u8) -> bool) {
        loop {
            self.cycle();
            if !condition(self.get_value()) {
                break;
            }
        }
    }

    #[must_use]
    pub fn get_value(&self) -> u8 {
        static RNG_VALUES: Lazy<[u8; Random::RNG_STATES_COUNT as usize]> = Lazy::new(|| {
            let mut values = [0; Random::RNG_STATES_COUNT as usize];

            let mut current: u16 = 0x8988;
            for value in values.iter_mut() {
                *value = (current >> 8) as u8;

                let new_bit = ((current >> 9) ^ (current >> 1)) & 1;
                current = (new_bit << 15) | (current >> 1);
            }
            values
        });

        RNG_VALUES[usize::from(self.index)]
    }

    pub fn get_piece(&mut self) -> Piece {
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

        self.piece_counter = (self.piece_counter + 1) % 8;
        let mut piece_index = u8::wrapping_add(self.get_value(), self.piece_counter) % 8;
        if (piece_index as usize) >= PIECE_TABLE.len() || get_piece(piece_index) == self.last_piece
        {
            self.cycle();
            piece_index = ((self.get_value() % 8) + self.last_piece as u8) % 7;
        }

        self.last_piece = get_piece(piece_index);

        self.last_piece
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}
