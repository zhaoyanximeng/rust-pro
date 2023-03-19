use crate::models::Book;
use crate::models::Phone;

pub trait Products{
    fn new(id:i32, price:f32) -> Self;
    fn get_price(&self)->f32;
}

impl Products for Book{
    fn new(id: i32, price: f32) -> Book {
        Book{id, price}
    }
    fn get_price(&self) -> f32 {
        &self.price + 10.0
    }
}

impl Products for Phone{
    fn new(id: i32, price: f32) -> Self {
        Phone{id, price}
    }

    fn get_price(&self) -> f32 {
        &self.price + 20.0
    }
}

// 重载+
impl std::ops::Add<Book> for Book {
    type Output = f32;

    fn add(self, rhs: Book) -> f32 {
        self.get_price() + rhs.get_price()
    }
}