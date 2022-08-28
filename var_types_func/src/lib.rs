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
    use std::collections::HashMap;
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

        //Tuples
        let tuple = ("hello", 5, 'c');
        println!("tuple is {:?}", tuple);

        assert_eq!(tuple.0, "hello");
        assert_eq!(tuple.2, 'c');

        /* let tuple = (20, 25, 30, "Rust", true, (1,2));
        println!("{} {} {} {}", tuple.1, tuple.0, tuple.3, (tuple.5).1);

        let (a, b, c, _d, _e, _f) = tuple;
        println!("{} {} {}", a, b, c); */

        //Structs
        struct Person {
            name: String,
            age: u8,
            likes_organges: bool,
        }

        struct Point2D(u32, u32);

        // struct Unit;

        let person = Person {
            name: String::from("JM"),
            likes_organges: true,
            age: 37,
        };

        println!(
            "I´m {}, my age is {} and it´s {} I like oranges",
            person.name, person.age, person.likes_organges
        );

        let some_coordinates = Point2D(5, 2);
        println!("{} {}", some_coordinates.0, some_coordinates.1);

        /* let mut bg_color = Color {
               red: 1,
               green: 2,
               blue: 3
           };

           bg_color.blue = 50;

           println!("{} {} {}", bg_color.red, bg_color.green, bg_color.blue);
        */

        /* struct Color {
            red: u8, //u8: 0-255
            green: u8,
            blue: u8
        } */

        /* struct Color(u8, u8, u8);

        fn print_color(c: &Color){
            println!("{} {} {}", c.0, c.1, c.2);
        } */

        /* let mut new_color = Color(1,2,25);
        new_color.2 = 24;
        println!("{}", new_color.2); */

        /* let blue = Color (0,0,255);
        print_color(&blue); */

        //Enums

        // Define a tuple struct
        #[derive(Debug)]
        struct KeyPress(String, char);
        // Define a classic struct
        #[derive(Debug)]
        struct MouseClick {
            x: i64,
            y: i64,
        }

        // Define the WebEvent enum variants to use the data from the structs
        // and a boolean type for the page Load variant
        #[derive(Debug)]
        enum WebEvent {
            WELoad(bool),
            WEClick(MouseClick),
            WEKeys(KeyPress),
        }

        // Instantiate a MouseClick struct and bind the coordinate values
        let click = MouseClick { x: 100, y: 250 };
        println!("Mouse click location: {}, {}", click.x, click.y);

        // Instantiate a KeyPress tuple and bind the key values
        let keys = KeyPress(String::from("Ctrl+"), 'N');
        println!("\nKeys pressed: {}{}", keys.0, keys.1);

        // Instantiate WebEvent enum variants
        // Set the boolean page Load value to true
        let we_load = WebEvent::WELoad(true);
        // Set the WEClick variant to use the data in the click struct
        let we_click = WebEvent::WEClick(click);
        // Set the WEKeys variant to use the data in the keys tuple
        let we_key = WebEvent::WEKeys(keys);

        // Print the values in the WebEvent enum variants
        // Use the {:#?} syntax to display the enum structure and data in a readable form
        println!(
            "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
            we_load, we_click, we_key
        );

        /* enum Direction {
            Up,
            Down,
            Left,
            Right
        } */

        /* let player_direction:Direction = Direction::Left;

        match player_direction {
            Direction::Up => println!("We are heading up!"),
            Direction::Down => println!("We are heading down!"),
            Direction::Left | Direction::Right => println!("Left or right!"),

        } */

        //Arrays

        // Declare array, initialize all values, compiler infers length = 7
        let days = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];
        println!("Selected day: {}", days[1]);

        // Declare array, initialize all values to 0, length = 5
        let bytes = [0; 5];
        println!("Byte content:{}", bytes[1]);
        /* let numbers: [i32;5] = [1, 2, 3, 4, 5];
        let array = [2;400];

        for n in numbers.iter() {
           println!("{}",n);
        }

        for i in 0..array.len() {
           println!("{}",array[i]);
        } */

        //Vectors

        // Declare vector, initialize with three values
        let three_nums = vec![15, 3, 46];
        println!("Initial vector: {:?}", three_nums);

        // Declare vector, value = "0", length = 5
        let zeroes = vec![0; 5];
        println!("Zeroes: {:?}", zeroes);

        // Create empty vector, declare vector mutable so it can grow and shrink
        let mut fruit = Vec::new();
        fruit.push("Apple");
        println!("Fruits: {:?}", fruit);

        /* let _my_vector: Vec<i32> = Vec::new();
        let mut my_vector2 = vec![1,2,3,4];
        println!("{}", my_vector2[3]);
        my_vector2.push(40);
        println!("{}", my_vector2[4]);
        my_vector2.remove(1);

        for number in my_vector2.iter(){
            println!("{}", number);
        } */

        /* let args: Vec<String> = env::args().collect();

        for argument in args.iter() {
            println!("{}", argument);
        }

        println!("{}", args[1]); */

        //HashMaps
        let mut reviews: HashMap<String, String> = HashMap::new();

        reviews.insert(
            String::from("Ancient Roman History"),
            String::from("Very accurate."),
        );
        // Look for a specific review
        let book: &str = "Programming in Rust";
        println!("\nReview for \'{}\': {:?}", book, reviews.get(book));
        /* let mut marks = HashMap::new();

        marks.insert("Rust Programmings", 96);
        marks.insert("Web", 85);
        marks.insert("UX Design", 80);
        marks.insert("Computing", 95);

        println!("{}", marks.len());

        match marks.get("Web"){
            Some(mark) => println!("Web: {}", mark),
            None => println!("You did not study web")
        }

        marks.remove("UX Design");

        for (subject,mark) in &marks {
            println!("{} : {}", subject, mark);
        }

        println!("{}", marks.contains_key("C++")); */
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

    pub mod strings {
        pub fn learn_string() {
            /* {
                let my_string = String::from("Rust ist fantastic");
                println!("After replace: {}", my_string.replace("fantastic", "great"));
            }

            {
                let my_string = String::from("Weather is \n nice\n outside");
                for line in my_string.lines(){
                    println!("[ {} ]", line);
                }
            }

            {
                let my_string = String::from("Leave+a+like+if+you+enoyed");
                let tokens: Vec<&str> = my_string.split("+").collect();
                println!("At index 2: {}", tokens[2]);
            }

            {
                let my_string = String::from("    My name is Jm Sar \n\r");
                println!("My string after trim: {}", my_string.trim());

            }
            {
                let my_string = String::from("On Youtube");
                match my_string.chars().nth(5){
                    Some(c) => println!("{}", c),
                    None => println!("No character at index 4")
                }

            } */
        }
    }
}
