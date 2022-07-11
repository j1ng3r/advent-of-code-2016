use std::env;
mod util;
mod puzzle1;
mod puzzle2;
fn main() {
   let args: Vec<String> = env::args().collect();
   let maybe_arg = args.get(1);
   match maybe_arg.map(String::as_str) {
      Some("1") => puzzle1::main(),
      Some("2") => puzzle2::main(),
      _ => {
         println!("Please run one of the two following commands:\ncargo run --bin {} 1\ncargo run --bin {} 2", util::DAY, util::DAY);
      },
   };
}
