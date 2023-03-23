use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Input: u8 {
        const Right = 0x01;
        const Left = 0x02;
        const Down = 0x04;
        const Up = 0x08;
        const Start = 0x10;
        const Select = 0x20;
        const B = 0x40;
        const A = 0x80;
    }
}

impl Input {
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

        Ok(Input::from_bits_retain(input_byte))
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::empty()
    }
}
