use super::util::Dir;
use super::util::get_input;

#[derive(Clone, Copy)]
struct State {
   x: i8,
   y: i8,
}
impl State {
   fn to_int(&self) -> i8 {
      return 1 + self.x + 3*self.y;
   }
   fn to_string(&self) -> String {
      return self.to_int().to_string();
   }
   fn clamp(v: i8) -> i8 {
      if v < 0 {
         return 0
      } else if v > 2 {
         return 2
      } else {
         return v
      }
   }
   fn consume_dir(&self, d: Dir) -> State {
      return match d {
         Dir::L => State {
            x: State::clamp(self.x - 1),
            y: self.y,
         },
         Dir::R => State {
            x: State::clamp(self.x + 1),
            y: self.y,
         },
         Dir::U => State {
            x: self.x,
            y: State::clamp(self.y - 1),
         },
         Dir::D => State {
            x: self.x,
            y: State::clamp(self.y + 1),
         },
      }
   }
   fn consume_dirs(&self, ds: Vec<Dir>) -> State {
      let mut state = *self;
      for d in ds {
         state = state.consume_dir(d);
      }
      return state;
   }
}
pub fn main() {
   let inputs: Vec<Vec<Dir>> = get_input();
   let mut state = State {
      x: 1,
      y: 1,
   };
   let mut code: String = String::from("");
   for ds in inputs {
      state = state.consume_dirs(ds);
      code += &state.to_string();
   }
   println!("The final code is {}.", code);
}
