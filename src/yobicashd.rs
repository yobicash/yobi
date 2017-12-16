extern crate yobicash_rs;
extern crate structopt;

use yobicash_rs::cli::node::*;
use structopt::StructOpt;

fn main() {
  let opt = YobicashNodeOpt::from_args();
  println!("yobicashd opt: {:?}", opt) 
}
