use super::util::get_input;
use super::util::option_from_result;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Instruction { L, R, F }

#[derive(Clone, Copy)]
enum Dir { D0, D1, D2, D3 }
impl Dir {
   pub fn consume_l(d: Dir) -> Dir {
      return match d {
         Dir::D0 => Dir::D3,
         Dir::D1 => Dir::D0,
         Dir::D2 => Dir::D1,
         Dir::D3 => Dir::D2,
      }
   }
   pub fn consume_r(d: Dir) -> Dir {
      return match d {
         Dir::D0 => Dir::D1,
         Dir::D1 => Dir::D2,
         Dir::D2 => Dir::D3,
         Dir::D3 => Dir::D0,
      }
   }
   pub fn to_string(d: Dir) -> &'static str {
      match d {
         Dir::D0 => "D0",
         Dir::D1 => "D1",
         Dir::D2 => "D2",
         Dir::D3 => "D3",
      }
   }
}


#[derive(Clone, Copy)]
struct State {
   x: i32,
   y: i32,
   dir: Dir,
   index: u32,
}
impl Hash for State {
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.x.hash(state);
      self.y.hash(state);
   }
}
impl PartialEq for State {
   fn eq(&self, other: &Self) -> bool {
         true
      && self.x == other.x
      && self.y == other.y
   }
}
impl Eq for State {}

impl State {
   pub fn consume_f(s: State) -> State {
      return match s.dir {
         Dir::D0 => State {
            x: s.x - 0,
            y: s.y - 1,
            dir: s.dir,
            index: s.index + 1,
         },
         Dir::D1 => State {
            x: s.x + 1,
            y: s.y + 0,
            dir: s.dir,
            index: s.index + 1,
         },
         Dir::D2 => State {
            x: s.x + 0,
            y: s.y + 1,
            dir: s.dir,
            index: s.index + 1,
         },
         Dir::D3 => State {
            x: s.x - 1,
            y: s.y - 0,
            dir: s.dir,
            index: s.index + 1,
         },
      }
   }
   pub fn consume(s: State, i: Instruction) -> State {
      return match i {
         Instruction::L => State {
            x: s.x,
            y: s.y,
            dir: Dir::consume_l(s.dir),
            index: s.index + 1,
         },
         Instruction::R => State {
            x: s.x,
            y: s.y,
            dir: Dir::consume_r(s.dir),
            index: s.index + 1,
         },
         Instruction::F => State::consume_f(s),
      }
   }
   pub fn dist_from_origin(s: State) -> u32 {
      let dist: i32 = s.x.abs() + s.y.abs();
      return dist.try_into().unwrap();
   }
   pub fn print(s: &State) {
      println!("State x = {}, state y = {}, state dir = {}, index = {}", s.x, s.y, Dir::to_string(s.dir), s.index);
   }
}

fn get_parsed_input() -> Vec<Instruction> {
   let input = get_input();
   let split = input.split(", ");
   let mut v = Vec::<Instruction>::new();
   for s in split {
      if s.starts_with('L') {
         v.push(Instruction::L);
      } else if s.starts_with('R') {
         v.push(Instruction::R);
      }
      let maybe_num_fs = option_from_result::<u32, ParseIntError>(s[1..].parse::<u32>());
      let num_fs = match maybe_num_fs {
         Some(num_fs) => num_fs,
         None => 0,
      };
      for _ in 0..num_fs {
         v.push(Instruction::F);
      }
   }
   return v;
}

pub fn main() {
   let instructions: Vec<Instruction> = get_parsed_input();
   let mut visited_positions = HashSet::<State>::new();
   let mut state = State {
      x: 0,
      y: 0,
      dir: Dir::D0,
      index: 0,
   };
   for instruction in instructions {
      visited_positions.insert(state);
      state = State::consume(state, instruction);
      if instruction == Instruction::F && visited_positions.contains(&state) {
         let dist = State::dist_from_origin(state);
         println!("The first crossing point has taxicab distance from the origin {}.", dist);
         State::print(&state);
         return;
      }
   }
   println!("Could not find a crossing point.");
}
