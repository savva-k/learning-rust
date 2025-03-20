fn main() {
    let number = 3;

    if number < 5 {
        println!("it was true");
    } else {
        println!("it was false");
    }

    let x = if true { 5 } else { 0 };

    println!("x = {x}");

    let z = foo();
    println!("z = {z}");


    for number in (1..4).rev() {
        println!("Num: {number}");
    }


}

fn foo() -> i32 {
    let mut cnt = 0;

    loop {
        cnt += 1;

        if cnt == 10 {
            break cnt * 2;
        }
    }
}
