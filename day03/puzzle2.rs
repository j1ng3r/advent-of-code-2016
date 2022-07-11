use super::util;
use util::Trpl;
use util::get_input;

impl Trpl {
   fn new() -> Trpl {
      return Trpl {
         a: 0,
         b: 0,
         c: 0,
      };
   }
}

enum TrplPos { A, B, C }

fn get_parsed_input() -> Vec<Trpl> {
   let input = get_input();
   let splt = input.trim().split("\n");
   
   let mut v: Vec<Trpl> = Vec::new();
   
   let mut tp = TrplPos::A;
   let mut ta = Trpl::new();
   let mut tb = Trpl::new();
   let mut tc = Trpl::new();
   for s in splt {
      let maybe_t = Trpl::from_str(s);
      match maybe_t {
         Some(t) => match tp {
            TrplPos::A => {
               ta.a = t.a;
               tb.a = t.b;
               tc.a = t.c;
               tp = TrplPos::B;
            },
            TrplPos::B => {
               ta.b = t.a;
               tb.b = t.b;
               tc.b = t.c;
               tp = TrplPos::C;
            },
            TrplPos::C => {
               ta.c = t.a;
               tb.c = t.b;
               tc.c = t.c;
               v.push(ta);
               v.push(tb);
               v.push(tc);
               ta = Trpl::new();
               tb = Trpl::new();
               tc = Trpl::new();
               tp = TrplPos::A;
            },
         },
         None => {}
      }
   }
   return v;
}

pub fn main() {
   let num_valid_triangles = get_parsed_input().iter().filter(Trpl::ok).count();
   println!("The number of valid triangles is {}.", num_valid_triangles);
}
