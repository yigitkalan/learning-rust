pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn fail() {
    //     panic!("THIS FAILS");
    // }

    #[test]
    fn larger_can_cover_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 4,
        };
        let smaller = Rectangle {
            length: 2,
            width: 1,
        };
        assert!(larger.can_cover(&smaller))
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(
            !smaller.can_cover(&larger),
            "Smaller rectangle shouldn't be able to cover a larger one"
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(102);
    }

    #[test]
    fn it_really_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    //we can run this by cargo test expensive_test
    //or cargo test -- --ignored
    fn expensive_test() {
        // code that takes an hour to run
    }
}
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: i32,
    width: i32,
}

impl Rectangle {
    fn can_cover(&self, other: &Rectangle) -> bool {
        self.length >= other.length && self.width > other.width
    }
}
