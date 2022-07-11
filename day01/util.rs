use std::fs;

pub static DAY: &str = "day01";
static FILENAME: &str = "day01/input.txt";

pub fn get_input() -> String {
   let input: String = fs::read_to_string(FILENAME)
      .expect("Something went wrong reading the file");
   return String::from(input.trim());
}

pub fn option_from_result<T, E>(maybe_x: Result<T, E>) -> Option<T> {
   return match maybe_x {
      Ok(x) => Some(x),
      Err(_) => None
   }
}
