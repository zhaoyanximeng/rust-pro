use std::fmt::Formatter;
#[derive(Default)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "用户id是{}，用户年龄是{}，用户名是{}", self.id, self.age, self.name)
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