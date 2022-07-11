use super::util::Dir;
use super::util::get_input;

#[derive(Clone, Copy)]
struct State {
   x: i8,
   y: i8,
}
impl State {
   fn print(&self) {
      println!("x = {}, y = {}", self.x, self.y);
   }
   fn to_string(&self) -> String {
      return match self.y {
         0 => (self.x - 1).to_string(),
         1 => (self.x + 1).to_string(),
         2 => (self.x + 5).to_string(),
         3 => match self.x {
            1 => String::from("A"),
            2 => String::from("B"),
            3 => String::from("C"),
            _ => String::from(""),
         },
         4 => String::from("D"),
         _ => String::from(""),
      }
   }
   fn in_bounds(&self) -> bool {
      return true
      && self.x + self.y >= 2
      && self.x + self.y <= 6
      && self.x - self.y >= -2
      && self.x - self.y <= 2
   }
   fn maybe_update(&self, new: State) -> State {
      if new.in_bounds() {
         return new;
      } else {
         return *self;
      }
   }
   fn consume_dir(&self, d: Dir) -> State {
      return self.maybe_update(match d {
         Dir::L => State {
            x: self.x - 1,
            y: self.y,
         },
         Dir::R => State {
            x: self.x + 1,
            y: self.y,
         },
         Dir::U => State {
            x: self.x,
            y: self.y - 1,
         },
         Dir::D => State {
            x: self.x,
            y: self.y + 1,
         },
      });
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
      x: 0,
      y: 2,
   };
   let mut code: String = String::from("");
   for ds in inputs {
      state = state.consume_dirs(ds);
      state.print();
      code += &state.to_string();
   }
   println!("The final code is {}.", code);
}
