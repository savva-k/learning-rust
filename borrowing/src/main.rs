fn main() {
    let s = String::from("test");
    takes_ownership(s);

    let x = b' ';
    println!("{x}");

    // error: 
    // println!("s = {s}");
}

fn takes_ownership(some_str: String) {
    println!("{some_str}");
}
