use bitvec::prelude::*;

#[derive(Clone, Eq, PartialEq)]
pub struct Input {
    pub states: BitArr!(for 8, in u8),
}
pub enum Button {
    Right = 0,
    Left = 1,
    Down = 2,
    Up = 3,
    Start = 4,
    Select = 5,
    B = 6,
    A = 7,
}

impl Input {
    pub fn new() -> Self {
        Self {
            states: BitArray::new([0]),
        }
    }

    pub fn from_fm2_string(string: String) -> Result<Input, String> {
        if string.len() != 8 {
            return Err("cannot create input from fm2 string of length != 8".into());
        }

        let mut input_byte = 0;
        for character in string.chars() {
            let state = character != ' ' && character != '.';
            input_byte >>= 1;
            input_byte |= if state { 0x80 } else { 0 };
        }

        Ok(Input {
            states: BitArray::new([input_byte]),
        })
    }

    pub fn get(&self, button: Button) -> bool {
        self.states[button as usize]
    }

    pub fn set(&mut self, button: Button, state: bool) {
        self.states.set(button as usize, state);
    }

    pub fn difference(&self, other: &Input) -> Input {
        Input {
            states: self.states & !other.states,
        }
    }
}
