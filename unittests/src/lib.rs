use core::panic;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn subtract(left: u64, right: u64) -> u64 {
    //panic!("Aaaaaa!")
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        subtract(0, 0);
    }
}
