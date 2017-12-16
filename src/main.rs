extern crate yobicash_rs;
extern crate structopt;

use yobicash_rs::cli::server::*;
use structopt::StructOpt;

fn main() {
  let opt = YobicashdOpt::from_args();
  println!("yobicashd opt: {:?}", opt) 
}
