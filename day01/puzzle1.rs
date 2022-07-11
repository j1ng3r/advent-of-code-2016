use std::vec::Vec;
use std::num::ParseIntError;
use super::util::get_input;
use super::util::option_from_result;

enum Turn { L, R }
impl Turn {
   pub fn from(c: Option<char>) -> Option<Turn> {
      return match c {
         Some('R') => Some(Turn::R),
         Some('L') => Some(Turn::L),
         _ => None,
      }
   }
}

struct Instruction {
   pub turn: Turn,
   pub dist: u32,
}

fn parse_input(input: &str) -> Vec<Instruction> {
   let split = input.split(", ");
   let res
      = split
      .map(|a| Some(Instruction {
         turn: Turn::from(a.chars().nth(0))?,
         dist: option_from_result::<u32, ParseIntError>(a[1..].parse::<u32>())?,
      }))
      .filter(|x| x.is_some())
      .map(|x| x.unwrap());
   return res.collect();
}

fn get_parsed_input() -> Vec<Instruction> {
   let input: String = get_input();
   let parsed_input = parse_input(&input);
   return parsed_input;
}
#[derive(Copy, Clone)]
enum Dir { D0, D1, D2, D3 }
impl Dir {
   pub fn consume(d: Dir, t: Turn) -> Dir {
      return match t {
         Turn::L => match d {
            Dir::D0 => Dir::D3,
            Dir::D1 => Dir::D0,
            Dir::D2 => Dir::D1,
            Dir::D3 => Dir::D2,
         },
         Turn::R => match d {
            Dir::D0 => Dir::D1,
            Dir::D1 => Dir::D2,
            Dir::D2 => Dir::D3,
            Dir::D3 => Dir::D0,
         },
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

struct Diff {
   x: i32,
   y: i32,
}
impl Diff {
   pub fn from(dir: Dir, dist: i32) -> Diff {
      return match dir {
         Dir::D0 => Diff {
            x: 0,
            y: -dist,
         },
         Dir::D1 => Diff {
            x: dist,
            y: 0,
         },
         Dir::D2 => Diff {
            x: 0,
            y: dist,
         },
         Dir::D3 => Diff {
            x: -dist,
            y: 0,
         },
      }
   }
}

struct State {
   x: i32,
   y: i32,
   dir: Dir,
}
impl State {
   pub fn consume(s: State, i: Instruction) -> State {
      let new_dir = Dir::consume(s.dir, i.turn);
      let diff = Diff::from(new_dir, i.dist.try_into().unwrap());
      let new_s = State {
         x: s.x + diff.x,
         y: s.y + diff.y,
         dir: new_dir,
      };
      return new_s;
   }
   pub fn from_instructions(instructions: Vec<Instruction>) -> State {
      let mut state = State {
         x: 0,
         y: 0,
         dir: Dir::D0,
      };
      for instruction in instructions {
         state = State::consume(state, instruction);
      }
      return state;
   }
   pub fn dist_from_origin(s: State) -> u32 {
      let dist: i32 = s.x.abs() + s.y.abs();
      return dist.try_into().unwrap();
   }
   pub fn print(s: &State) {
      println!("State x = {}, state y = {}, state dir = {}", s.x, s.y, Dir::to_string(s.dir));
   }
}

pub fn main() {
   let instructions: Vec<Instruction> = get_parsed_input();
   let state: State = State::from_instructions(instructions);
   State::print(&state);
   
   let dist = State::dist_from_origin(state);
   println!("The final taxicab distance from the origin is {}.", dist);
}
