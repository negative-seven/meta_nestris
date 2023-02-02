use bitmask_enum::bitmask;

#[bitmask(u8)]
pub enum Input {
    None = 0,
    Right = 0x01,
    Left = 0x02,
    Down = 0x04,
    Up = 0x08,
    Start = 0x10,
    Select = 0x20,
    B = 0x40,
    A = 0x80,
}

impl Input {
    #[must_use]
    pub fn new() -> Self {
        Self::None
    }

    pub fn from_fm2_string(string: &String) -> Result<Input, String> {
        if string.len() != 8 {
            return Err("cannot create input from fm2 string of length != 8".into());
        }

        let mut input_byte = 0;
        for character in string.chars() {
            let state = character != ' ' && character != '.';
            input_byte >>= 1;
            input_byte |= if state { 0x80 } else { 0 };
        }

        Ok(Input::from(input_byte))
    }

    #[must_use]
    pub fn get(self, button: Input) -> bool {
        self & button != 0
    }

    #[must_use]
    pub fn get_only_input(self, button: Input) -> bool {
        self == button
    }

    pub fn set(mut self, button: Input, state: bool) {
        if state {
            self |= button;
        } else {
            self &= !button;
        }
    }

    #[must_use]
    pub fn difference(self, other: Input) -> Input {
        self & !other
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
