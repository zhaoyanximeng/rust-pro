mod lib;
use lib::prelude::CommonInit;
use lib::user::*;

fn main() {
   let u = User::new("zyxm");
    println!("{}", u);
}
