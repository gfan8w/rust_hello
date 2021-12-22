///演示 workspace


use rand;

pub fn add_one(x: i32) -> i32 {
    x + rand::random::<i32>()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(6);
        assert_eq!(result, 7);
    }
}


