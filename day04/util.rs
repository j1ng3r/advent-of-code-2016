use std::collections::HashMap;

pub static DAY: &str = "day04";
static FILENAME: &str = "day04/input.txt";

pub fn to_u32(s: &str) -> Option<u32> {
   return s.parse::<u32>().ok();
}

// Ok I'm done for the night

// pub fn collect_options<T>(i: impl Iterator<Item = Option<T>>) -> Vec<T> {
//    return i.filter(|x| x.is_some()).map(|x| x.unwrap()).collect();
// }

pub fn str_to_hashmap(s: &str) -> HashMap<char, u32> {
   let mut h: HashMap<char, u32> = HashMap::new();
   for c in s.chars() {
      if c != '-' {
         let current_count: u32 = match h.get(&c) {
            Some(&count) => count,
            None => 0,
         };
         h.insert(c, current_count + 1);
      }
   }
   return h;
}

pub fn get_inputs() -> String {
   let input: String = std::fs::read_to_string(FILENAME)
      .expect("Something went wrong reading the file");
   return input;
}
