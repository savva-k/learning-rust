
struct User {
    name: String,
    age: u8,
}

// Pub: the trait should be used in place where its methods are called!
pub trait Greet {
    fn greet(&self) -> String;
}

impl Greet for User {
    fn greet(&self) -> String {
        format!("Hello, I am {} ({})", &self.name, &self.age)
    }
}

fn get_greet(user: &User) -> String {
    format!("{}", user.greet())
}

fn main() {
    let user = User { name: String::from("Savva"), age: 37 };
    println!("{}", get_greet(&user));
}
