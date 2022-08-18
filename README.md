# meta_nestris

A largely functionally accurate recreation of NES Tetris in Rust.

### Intentional differences from the base game:
  - although they ultimately result in the same state, some operations have been reordered to make the code cleaner and/or faster
  - the score is not capped at 999999 and is now subject to 32-bit overflow instead
  
### Game events which are not intended to be supported:
  - reaching the demo
  - console resets
  - the in-game reset code
  - anything that happens after death or a B-type clear
  - lag/crashing at high levels
  
### Known untested behavior:
  - overflow for some variables, such as line count or level number.
