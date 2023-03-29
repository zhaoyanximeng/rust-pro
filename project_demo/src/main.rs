mod lib;
use lib::prelude::UserInit;
use lib::user::*;

fn main() {
   // let u = User::new("zyxm");
   //  println!("{}", serde_json::to_string(&u).unwrap());
    let user_str = r#"{
        "id": 0,
        "user_name": "zyxm"
    }"#;
    let user:User = serde_json::from_str(user_str).unwrap();
    println!("{}", user)
}
