pub mod arrays;
pub mod enums;
pub mod hashmaps;
pub mod strings;
pub mod structs;
pub mod tuples;
pub mod vectors;

pub mod variables {
    pub fn learn_variables() {
        // This is a line comment Ctrl + Shift + 7
        /* This is a block comment
        with 2 lines Ctrl + Shift + A */
        println!("It works");
        // The `mut` keyword lets the variable be changed
        let mut a_number = 10;
        let a_boolean = true;

        println!("the number is {}", a_number);
        println!("the boolean is {}", a_boolean);

        a_number = 15;

        println!("Mutability: the number is {}", a_number);

        // Declare second variable binding, shadows existing variable "b_number"
        let b_number = 5;
        let b_number = b_number * 5;

        println!("Shadowing: b_number is {}", b_number);

        println!("Hello, world!");

        let mut x: u32 = 45;
        let f: f32 = 6.7;
        let b: bool = false;
        println!("x is {}, f is {}, b is {}", x, f, b);

        x = 60;

        println!("x is {}", x);
    }
}

pub mod types {

    pub fn learn_types() {
        //Starting with Types
        // let x = 2.0;
        // let y: f32 = 3.0;

        // let is_bigger = 1 > 4;
        // println!("{}", is_bigger);

        //char, String, &str
        let mut hello = String::from("Hello, ");
        hello.push('w');
        hello.push_str("orld");
        println!("{}", hello);

        /* let mut my_string = String::from("Hello o o");
        println!("{}", my_string.len());
        println!("{}", my_string.is_empty());

        for word in my_string.split_whitespace(){
            println!("{}", word);
        }

        my_string.push_str("Hello!");
        println!("{}", my_string); */
    }
}

pub mod functions {

    pub fn learn_functions() {
        //Calling a function

        assert_eq!(divide_by_2(4, 2), true);
    }

    fn divide_by_2(dividend: u32, divisor: u32) -> bool {
        if dividend == 0 {
            return false;
        }
        dividend % divisor == 0
    }
}

pub mod conditionals {
    pub fn learn_conditionals() {
        let n = 20;

        if n < 30 {
            println!("The number is less than 30");
        }

        let learning: bool = false;

        if learning {
            println!("I'm learning Rust");
        } else {
            println!("I'm not learning Rust");
        }
    }

    pub fn learn_pattern_matching() {
        /* let number = 5;

        match number {
            1 => println!("It is one"),
            2..=5 => println!("It's a range"),
            10 | 11 => println!("It's either 10 or 11"),
            _ => println!("No match")
        } */

        /* let mut input = String::new();
           println!("Write something:");

           match io::stdin().read_line(&mut input) {
               Ok(_) => {
                   println!("Success! {}", input);
               },
               Err(e) => println!("Error: {}", e)
           }
        */
        /* let name = String::from("Jmsar");

        println!("{}", match name.chars().nth(1){
           Some(c) => c.to_string(),
           None => "No character at index 8!".to_string()
        }) */

        /* println!("Occupation is {}", match get_occupation("Jmsar"){
           Some(o) => o,
           None => "No occuption found"
        }); */
        /* fn get_occupation(name: &str) -> Option<&str> {
           match name {
               "Jmsar" => Some("Software Developer"),
               "Michael" => Some("Dentist"),
               _ => None
           }
        } */
    }
}
