use serde::{Serialize, Deserialize, Serializer};
use std::fmt::Formatter;
use serde::ser::SerializeStruct;

#[derive(Debug,Clone)]
pub struct User {
    pub id: i32,
    // #[serde(rename(serialize="user_name", deserialize="user_name"))]
    pub name: String,
    // #[serde(default="User::default_age")]
    pub age: i32,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("User", 3)?;
        s.serialize_field("name",&self.name)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("age", &self.age)?;
        s.end()
    }
}

impl User{
    fn default_age()->i32 {
        18
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "用户id是{}，用户年龄是{}，用户名是{}", self.id, self.age, self.name)
    }
}

impl Default for User {
    fn default() -> Self {
        User{id:0, name:String::from("zyxm1"), age: 18}
    }
}

use super::prelude::*;

impl UserInit<i32> for User {
    fn new(v: i32) -> Self {
        return User{id: v, ..Default::default()}
    }
}

impl UserInit<&str> for User {
    fn new(v: &str) -> Self {
        return User{name: v.to_string(), ..Default::default()}
    }
}

impl CommonInit for User {
    fn new() -> Self {
        User{..Default::default()}
    }
}