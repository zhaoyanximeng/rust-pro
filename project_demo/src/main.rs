mod lib;
use lib::prelude::UserInit;
use lib::user::*;

fn main() {
   // let u = User::new("zyxm");
   //  println!("{}", serde_json::to_string(&u).unwrap());
   //  let user_str = r#"{
   //      "id": 0,
   //      "user_name": "zyxm"
   //  }"#;
   //  let user:User = serde_json::from_str(user_str).unwrap();
   //  println!("{}", user)

    let user_str = r#"{
        "id":0,
        "user_name":"zyxm",
        "friends":["coleary", "faith"],
        "roles":[{"name":"admin"},{"name":"zyxm"}]
    }"#;
    let user:serde_json::Value = serde_json::from_str(user_str).unwrap();
    println!("{}", user.as_object().and_then(|v| v.get("friends")).and_then(|v|v.get(0)).unwrap());
}
