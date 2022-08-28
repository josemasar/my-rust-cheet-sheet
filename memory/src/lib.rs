pub mod borrow_checker {

    pub fn learn_borrowchecker() {
        // The concept of ownership in Rust.
        // Moving and borrowing values.
        // Scoping rules and lifetimes.
        // Rust's pointer types, commonly called references.

        // fn process(_input: u32) {}

        #[derive(Debug)]
        struct Highlight<'document>(&'document str);

        fn print_greeting(message: &String) {
            println!("Greeting: {}", message);
        }

        fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        //Copying instead of moving
        // let n = 1u32;
        // process(n); // Ownership of the number in `n` copied into `process`
        // process(n); // `n` can be used again because it wasn't moved, it was copied.

        //clones -> expensive operations -> better borrowing
        // let s = String::from("Hello, world!");
        // process(s.clone()); // Passing another value, cloned from `s`.
        // process(s); // s was never moved and so it can still be used.

        // This type of functionality is available by using references.
        // References allow us to "borrow" values without taking ownership of them.

        let greeting = String::from("Hello");
        print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
        print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again

        //With & borrows, known as "immutable borrows," we can read the data but we can't change it.
        //With &mut borrows, known as "mutable borrows," we can both read and write the data.

        /* Your code must implement either of the following definitions, but not both at the same time:
        One or more immutable references (&T)
        Exactly one mutable reference (&mut T) */

        //Validate references by using lifetimes
        let magic1 = String::from("abracadabra!");
        let magic2 = String::from("shazam!");

        let result = longest_word(&magic1, &magic2);
        println!("The longest magic word is {}", result);

        //Annotating lifetimes in types
        let text = String::from("The quick brown fox jumps over the lazy dog.");
        let fox = Highlight(&text[4..19]);
        let dog = Highlight(&text[35..43]);
        println!("{:?}", fox);
        println!("{:?}", dog);
    }
}
