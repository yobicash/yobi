extern crate yobicash_rs;
extern crate structopt;

use yobicash_rs::cli::client::*;
use structopt::StructOpt;

fn main() {
  let opt = YobicashClientOpt::from_args();
  println!("yobicash client opt: {:?}", opt) 
}
