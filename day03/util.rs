pub static DAY: &str = "day03";
static FILENAME: &str = "day03/input.txt";

fn option_from_result<T, E>(maybe_x: Result<T, E>) -> Option<T> {
   return match maybe_x {
      Ok(x) => Some(x),
      Err(_) => None
   }
}

pub fn to_u32(s: &str) -> Option<u32> {
   return option_from_result::<u32, std::num::ParseIntError>(s.parse::<u32>());
}

pub fn collect_options<T>(i: impl Iterator<Item = Option<T>>) -> Vec<T> {
   return i.filter(|x| x.is_some()).map(|x| x.unwrap()).collect();
}

pub struct Trpl {
   pub a: u32,
   pub b: u32,
   pub c: u32,
}
impl Trpl {
   pub fn from_str(s: &str) -> Option<Trpl> {
      let abc: Vec<&str> = s.trim().split(" ").filter(|s| !s.is_empty()).collect();
      return Some(Trpl {
         a: to_u32(abc.get(0)?)?,
         b: to_u32(abc.get(1)?)?,
         c: to_u32(abc.get(2)?)?
      });
   }
   pub fn ok(t: &&Trpl) -> bool {
      return true
      && t.a + t.b > t.c
      && t.b + t.c > t.a
      && t.c + t.a > t.b
   }
}

pub fn get_input() -> String {
   let input: String = std::fs::read_to_string(FILENAME)
      .expect("Something went wrong reading the file");
   return input;
}
