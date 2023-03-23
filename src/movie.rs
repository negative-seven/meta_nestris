use crate::input::Input;
use regex::Regex;
use static_init::dynamic;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

pub struct Movie {
    pub inputs: Vec<Input>,
}

impl Movie {
    pub fn from_fm2(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        #[dynamic]
        static REGEX_INPUT: Regex = Regex::new(r"^\|\d+\|([^|]{8})\|").unwrap();

        let file = File::open(path)?;
        let lines = BufReader::new(file)
            .lines()
            .collect::<Result<Vec<String>, io::Error>>()?
            .into_iter()
            .skip_while(|l| !l.starts_with('|'));

        let mut inputs = Vec::new();
        for line in lines {
            match REGEX_INPUT.captures(&line) {
                Some(captures) => {
                    let input = Input::from_fm2_string(&captures.get(1).unwrap().as_str().into())?;
                    inputs.push(input);
                }
                None => return Err("non-input line in fm2 input log section".into()),
            }
        }

        Ok(Movie { inputs })
    }
}
