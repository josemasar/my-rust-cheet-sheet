pub mod generic_types_traits {
    pub fn learn_generictypestraits() {
        /* What generic types are and how "wrapper" types use them.
        What traits are and how they help us define shared behavior.
        How to implement an existing trait for a custom type.
        How to implement a custom trait for an existing type.
        How trait bounds help us write generic functions.
        How to implement an Iterator trait to iterate through collections.
        */

        /* A trait is a common interface that a group of types can implement.
        The Rust standard library has many useful traits, such as:

        io::Read for values that can read bytes from a source.
        io::Write for values that can write out bytes.
        Debug for values that can be printed in the console using the "{:?}"
        format specifier.
        Clone for values that can be explicitly duplicated in memory.
        ToString for values that can be converted to a String.
        Default for types that have a sensible default value, like zero for numbers,
        empty for vectors, and “” for String.
        Iterator for types that can produce a sequence of values.

        Each trait definition is a collection of methods defined for an unknown type,
        usually representing a capability or behavior that its implementors can do. */

        // struct Point<T, U> {
        //     x: T,
        //     y: U,
        // }

        //Custom Types can't be compared to other instances or displayed in the terminal.

        /* The Debug trait, which allows a type to be formatted by using the {:?} format specifier, is used in a programmer-facing, debugging context.
        The Display trait, which allows a type to be formatted by using the {} format specifier, is similar to Debug. But Display is better suited for user-facing output.
        The PartialEq trait, which allows implementors to be compared for equality. */
        /* use std::fmt;

        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        } */

        /* trait Area {
            fn area(&self) -> f64;
        }

        struct Circle {
            radius: f64,
        }

        struct Rectangle {
            width: f64,
            height: f64,
        }

        impl Area for Circle {
            fn area(&self) -> f64 {
                use std::f64::consts::PI;
                PI * self.radius.powf(2.0)
            }
        }

        impl Area for Rectangle {
            fn area(&self) -> f64 {
                self.width * self.height
            }
        }

        let circle = Circle { radius: 5.0 };
        let rectangle = Rectangle {
            width: 10.0,
            height: 20.0,
        };

        println!("Circle area: {}", circle.area());
        println!("Rectangle area: {}", rectangle.area()); */

        // Use the derive trait

        /* impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 4, y: -3 };

        if p1 == p2 { // can't compare two Point values!
            println!("equal!");
        } else {
            println!("not equal!");
        }

        println!("{}", p1); // can't print using the '{}' format specifier!
        println!("{:?}", p1); //  can't print using the '{:?}' format specifier! */

        trait AsJson {
            fn as_json(&self) -> String;
        }

        struct Person {
            name: String,
            age: u8,
            favorite_fruit: String,
        }

        impl AsJson for Person {
            fn as_json(&self) -> String {
                format!(
                    r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
                    self.name, self.age, self.favorite_fruit
                )
            }
        }

        let laura = Person {
            name: String::from("Laura"),
            age: 31,
            favorite_fruit: String::from("apples"),
        };

        send_data_as_json(&laura);

        fn send_data_as_json(value: &impl AsJson) {
            println!("Sending JSON data to server...");
            println!("-> {}", value.as_json());
            println!("Done!\n");
        }
    }
}
