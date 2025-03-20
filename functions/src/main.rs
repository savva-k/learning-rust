fn main() {
    another_function(4);
    let y = {
        let y = 5;
        y + 2
    };

    println!("y = {y}");

    let z = four();
    println!("z = {z}");
}

fn another_function(x: i32) {
    println!("x = {x}");
}

fn four() -> i32 {
    4
}
