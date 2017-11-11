pub mod geometry {
    #[derive(Debug)]
    pub struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Self) -> bool {
            self.length > other.length && self.width > other.width
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle {
                length: 8,
                width: 7,
            };
            let smaller = Rectangle {
                length: 5,
                width: 1,
            };

            assert!(larger.can_hold(&smaller))

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

            assert!(!smaller.can_hold(&larger))
        }
    }
}

pub mod math {
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
        }
    }
}

pub mod game {
    pub struct Guess {
        pub value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Self {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {:?}", value);
            }
            Guess { value }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic(expected = "Guess value must be between 1 and 100, got 200")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
}
