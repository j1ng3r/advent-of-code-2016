use std::fs;

pub static DAY: &str = "day02";
static FILENAME: &str = "day02/input.txt";

pub enum Dir { L, R, U, D }
impl Dir {
   fn from_char(c: char) -> Option<Dir> {
      return match c {
         'L' => Some(Dir::L),
         'R' => Some(Dir::R),
         'U' => Some(Dir::U),
         'D' => Some(Dir::D),
         _ => None,
      }
   }
   fn from_str(s: &str) -> Vec<Dir> {
      let mut v: Vec<Dir> = Vec::new();
      for c in s.chars() {
         match Dir::from_char(c) {
            Some(dir) => v.push(dir),
            None => {}
         }
      }
      return v;
   }
}

pub fn get_input() -> Vec<Vec<Dir>> {
   let input: String = fs::read_to_string(FILENAME)
      .expect("Something went wrong reading the file");
   return input.trim().split("\n").map(Dir::from_str).collect();
}

pub fn option_from_result<T, E>(maybe_x: Result<T, E>) -> Option<T> {
   return match maybe_x {
      Ok(x) => Some(x),
      Err(_) => None
   }
}
