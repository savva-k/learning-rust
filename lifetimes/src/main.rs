fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let s1 = String::from("Test");
    let s2 = "Madest";
    println!("The longest str is: {}", longest(s1.as_str(), s2));
}
