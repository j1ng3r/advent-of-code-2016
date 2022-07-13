use regex::Regex;
use super::util;
use std::collections::HashSet;

struct Input {
   name: String,
   code: u32,
   order: String,
}
impl Input {
   fn from(input: &str) -> Option<Input> {
      let r = Regex::new(r"(?P<name>[-\w]+)-(?P<code>\d+)[(?P<order>\w+)]").ok()?;
      let caps = r.captures(input)?;
      
      return Some(Input {
         name: String::from(&caps["name"]),
         code: util::to_u32(&caps["code"])?,
         order: String::from(&caps["order"]),
      });
   }
   fn is_real_room(input: &Input) -> bool {
      // I could use a more efficient data structure here
      // but I don't feel like implementing it right now
      let h = util::str_to_hashmap(&input.name);
      let mut v: Vec<(&char, &u32)> = h.iter().collect();
      v.sort_by(|a, b| b.1.cmp(a.1));
      
      let mut i: usize = 0;
      let mut j: usize = 0;
      loop {
         let val = v.get(i).unwrap().1;
         let mut val_set : HashSet<char> = HashSet::new();
         while i < v.len() && v.get(i).unwrap().1 == val {
            val_set.insert(*v.get(i).unwrap().0);
            i += 1;
         }
         while val_set.len() > 0 {
            if j >= input.order.len() {
               return true;
            }
            let c = input.order.chars().nth(j).unwrap();
            if val_set.contains(&c) {
               val_set.remove(&c);
               j += 1
            } else {
               return false;
            }
         }
      }
   }
   fn is_real_room_opt(maybe_input: &Option<Input>) -> bool {
      return match maybe_input {
         Some(input) => Input::is_real_room(&input),
         None => false
      }
   }
}

pub fn main() {
   let code_sum: u32 = util::get_inputs()
      .trim()
      .split("\n")
      .map(Input::from)
      .filter(Input::is_real_room_opt)
      .fold(0 as u32, |x, y| x + y.unwrap().code);
   println!("Total sum = {}", code_sum);
}
