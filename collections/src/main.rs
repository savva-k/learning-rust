fn main() {
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![4, 5, 6];
    for i in &mut v {
        //*i += 5;
        println!("{}", *i + 5);
        println!("{}", *i);
    }
}
