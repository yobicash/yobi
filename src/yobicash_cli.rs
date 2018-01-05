extern crate structopt;
extern crate yobicash;

use structopt::StructOpt;
use yobicash::cli::client::*;

fn main() {
  let opt = YClientOpt::from_args();
  println!("yobicash client opt: {:?}", opt) 
}
