use super::util;
use util::Trpl;
use util::collect_options;
use util::get_input;



fn get_parsed_input() -> Vec<Trpl> {
   let input = get_input();
   return collect_options(input.trim().split("\n").map(Trpl::from_str));
}

pub fn main() {
   let num_valid_triangles = get_parsed_input().iter().filter(Trpl::ok).count();
   println!("The number of valid triangles is {}.", num_valid_triangles);
}
