#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    #[must_use]
    pub fn get_clockwise_rotation(self) -> Self {
        const CLOCKWISE_ROTATIONS: [Piece; 19] = {
            let mut rotations = [Piece::None; 19];

            let rotation_cycles = Piece::get_rotation_cycles();

            let mut cycle_index = 0;
            while cycle_index < rotation_cycles.len() {
                let cycle = rotation_cycles[cycle_index];

                let mut piece_index = 0;
                while piece_index < cycle.len() {
                    rotations[cycle[piece_index] as usize] = cycle[(piece_index + 1) % cycle.len()];

                    piece_index += 1;
                }

                cycle_index += 1;
            }

            rotations
        };

        CLOCKWISE_ROTATIONS[self as usize]
    }

    #[must_use]
    pub fn get_counterclockwise_rotation(self) -> Self {
        const COUNTERCLOCKWISE_ROTATIONS: [Piece; 19] = {
            let mut rotations = [Piece::None; 19];

            let rotation_cycles = Piece::get_rotation_cycles();

            let mut cycle_index = 0;
            while cycle_index < rotation_cycles.len() {
                let cycle = rotation_cycles[cycle_index];

                let mut piece_index = 0;
                while piece_index < cycle.len() {
                    rotations[cycle[piece_index] as usize] =
                        cycle[(piece_index + cycle.len() - 1) % cycle.len()];

                    piece_index += 1;
                }

                cycle_index += 1;
            }

            rotations
        };

        COUNTERCLOCKWISE_ROTATIONS[self as usize]
    }

    #[must_use]
    pub fn get_tile_offsets(self) -> &'static [(i8, i8); 4] {
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

        &TILE_OFFSETS[self as usize]
    }

    const fn get_rotation_cycles() -> &'static [[Piece; 4]; 7] {
        const ROTATION_CYCLES: [[Piece; 4]; 7] = {
            [
                [Piece::TUp, Piece::TRight, Piece::TDown, Piece::TLeft],
                [Piece::JUp, Piece::JRight, Piece::JDown, Piece::JLeft],
                [
                    Piece::ZHorizontal,
                    Piece::ZVertical,
                    Piece::ZHorizontal,
                    Piece::ZVertical,
                ],
                [Piece::O, Piece::O, Piece::O, Piece::O],
                [
                    Piece::SHorizontal,
                    Piece::SVertical,
                    Piece::SHorizontal,
                    Piece::SVertical,
                ],
                [Piece::LUp, Piece::LRight, Piece::LDown, Piece::LLeft],
                [
                    Piece::IHorizontal,
                    Piece::IVertical,
                    Piece::IHorizontal,
                    Piece::IVertical,
                ],
            ]
        };

        &ROTATION_CYCLES
    }
}
