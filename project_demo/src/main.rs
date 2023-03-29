mod lib;
use lib::prelude::UserInit;
use lib::user::*;

fn main() {
   let u = User::new("zyxm");
    println!("{}", u);
}
