pub mod error_handling {
    pub fn learn_errors() {
        /*Use panic! to deal with unrecoverable errors.
        Use the Option enum when a value is optional or the lack of a value is not an error condition.
        Use the Result enum when things could go wrong and a caller might have to deal with the problem.
        */

        #[derive(Debug)]
        struct DivisionByZeroError;

        fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
            if divisor == 0.0 {
                Err(DivisionByZeroError)
            } else {
                Ok(dividend / divisor)
            }
        }

        // let v = vec![0, 1, 2, 3];
        // println!("{}", v[6]); // this will cause a panic!

        let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

        // pick the first item:
        let first = fruits.get(0);
        println!("{:?}", first);

        // pick the third item:
        let third = fruits.get(2);
        println!("{:?}", third);

        // pick the 99th item, which is non-existent:
        let non_existent = fruits.get(99);
        println!("{:?}", non_existent);

        //Enum Option
        // enum Option<T> {
        //     None,     // The value doesn't exist
        //     Some(T),  // The value exists
        // }

        // But how can we access the data inside a Some(data) variant?

        for &index in [0, 2, 99].iter() {
            match fruits.get(index) {
                Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
                None => println!("There is no fruit! :("),
            }
        }
        //match arms must cover every possible value that the input type could have.
        //You'll get a compiler error if you try to match against a non-exhaustive pattern list.

        // let a_number: Option<u8> = Some(10);
        // match a_number {
        //     Some(7) => println!("That's my lucky number!"),
        //     _ => {},
        // }
        //You can add the _ (underscore) wildcard pattern after all other patterns to match anything else

        // let a_number: Option<u8> = Some(7);
        // if let Some(7) = a_number {
        //     println!("That's my lucky number!");
        // }

        //You can try to access the inner value of an Option type directly by using the unwrap method.
        // Be careful, though, because this method will panic if the variant is a None.

        // let empty_gift: Option<&str> = None;
        // assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!

        //The expect method does the same as unwrap,
        //but it provides a custom panic message that's provided by its second argument:
        // assert_eq!(empty_gift.expect("no present"),"car");

        //Better approach:
        //Use pattern matching and handle the None case explicitly.
        //Call similar non-panicking methods, such as unwrap_or, which returns a default value
        //if the variant is None or the inner value if the variant is Some(value).
        assert_eq!(Some("dog").unwrap_or("cat"), "dog");
        assert_eq!(None.unwrap_or("cat"), "cat");

        //Enum Result
        // enum Result<T, E> {
        //     Ok(T):  // A value T was obtained.
        //     Err(E): // An error of type E was encountered instead.
        // }

        println!("{:?}", safe_division(9.0, 3.0));
        println!("{:?}", safe_division(4.0, 0.0));
        println!("{:?}", safe_division(0.0, 2.0));
    }
}
