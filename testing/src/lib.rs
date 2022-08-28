pub mod testing {
    pub fn learn_testing() {
        /* How to write unit tests.
        How to write integration tests.
        How to write documentation tests. */
    }
    #[cfg(test)]
    mod add_function_tests {

        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        #[test]
        fn add_works() {
            assert_eq!(add(1, 2), 3);
            assert_eq!(add(10, 12), 22);
            assert_eq!(add(5, -2), 3);
        }

        #[test]
        #[should_panic]
        fn add_fails() {
            assert_eq!(add(2, 2), 7);
        }

        #[test]
        #[ignore = "not yet reviewed"]
        fn add_negatives() {
            assert_eq!(add(-2, -2), -4)
        }
    }
}
