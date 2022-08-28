extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod notes {
    #![allow(dead_code)]

    /* use std::fs::File;
    use std::io::prelude::*; */

    /* use std::io; */

    /* use std::env; */

    /* use std::collections::HashMap; */

    /* extern crate rand;
    use rand::Rng; */

    /* mod jmsar; */

    /* extern crate regex;
    use regex::Regex; */

    /* use std::process::Command; */

    /* extern crate reqwest; */

    #[allow(unused_imports)]
    use serde_json::Value as JsonValue;

    #[derive(Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u8,
        is_male: bool,
    }

    /* mod jmsar2 {
        pub fn print_message(){
            chicken();
        println!("I am Jm Sar");
    }
        fn chicken(){
            println!("Chicken");
        }

        pub mod water {
            pub fn print_message(){
                println!("Water");
            }
        }
    } */

    pub fn learn_notes() {
        // This is a line comment Ctrl + Shift + 7
        /* This is a block comment
        with 2 lines Ctrl + Shift + A */

        /*  println!("Hello, world!");

           let mut x: u32 = 45;
           let f: f32 = 6.7;
           let b: bool = false;


           println!("x is {}, f is {}, b is {}", x, f, b);

           x = 60;

           println!("x is {}", x);

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

           let mut counter = 0;

           loop {
               counter += 1;

               if counter == 5 {
                   continue;
               }

               if counter > 10 {
                   break;
               }

               println!("{}", counter);
           }
        */
        /* let mut n = 1;

        while n <= 20 {

            if n % 5 == 0 {
                // n multiple of 5
                println!("n is {}", n);
            }

            n += 1;
        } */

        /* let numbers = 30..51;

        for i in numbers {
            println!("the number is {}", i);
        }

        let animals = vec!["Rabbit", "Dog", "Cat"];

        for (index, a) in animals.iter().enumerate(){
            println!("the {} animal is {}", index, a);
        } */

        /* let player_direction:Direction = Direction::Left;

        match player_direction {
            Direction::Up => println!("We are heading up!"),
            Direction::Down => println!("We are heading down!"),
            Direction::Left | Direction::Right => println!("Left or right!"),

        } */

        /* for n in 1..MAXIMUN_NUMBER {
            println!("{}", n);
        } */

        /* let tuple = (20, 25, 30, "Rust", true, (1,2));
        println!("{} {} {} {}", tuple.1, tuple.0, tuple.3, (tuple.5).1);

        let (a, b, c, _d, _e, _f) = tuple;
        println!("{} {} {}", a, b, c); */

        /* print_numbers_to(10);

        if is_even(6){
            println!("is even");
        } */

        /* let x = 10;

        {
            let y = 5;
            println!("x is {}, y is {}", x, y);
        }

        println!("x is {}, y is {}", x, y); */

        //References

        /* let mut x = 10;
        // let xr = &x;
        {
            let second_reference = &mut x;
            *second_reference += 1;
        }

        println!("{}", x); */

        /* let mut bg_color = Color {
               red: 1,
               green: 2,
               blue: 3
           };

           bg_color.blue = 50;

           println!("{} {} {}", bg_color.red, bg_color.green, bg_color.blue);
        */

        /* let mut new_color = Color(1,2,25);
        new_color.2 = 24;
        println!("{}", new_color.2); */

        /* let blue = Color (0,0,255);
        print_color(&blue); */

        /* let numbers: [i32;5] = [1, 2, 3, 4, 5];
        let array = [2;400];

        for n in numbers.iter() {
           println!("{}",n);
        }

        for i in 0..array.len() {
           println!("{}",array[i]);
        } */

        /* let my_rect = Rectangle { width:10, height:5 };
        my_rect.print_description();
        println!("{}", my_rect.is_square()) */

        /* let mut my_string = String::from("Hello o o");
        println!("{}", my_string.len());
        println!("{}", my_string.is_empty());

        for word in my_string.split_whitespace(){
            println!("{}", word);
        }

        my_string.push_str("Hello!");
        println!("{}", my_string); */

        /* let jm_sar = Person { name: String::from("Jm Sar"), age: 21};

        println!("{}", jm_sar.to_string()); */

        /* let _my_vector: Vec<i32> = Vec::new();
        let mut my_vector2 = vec![1,2,3,4];
        println!("{}", my_vector2[3]);
        my_vector2.push(40);
        println!("{}", my_vector2[4]);
        my_vector2.remove(1);

        for number in my_vector2.iter(){
            println!("{}", number);
        } */

        /* let mut file = File::open("info.txt").expect("Cant open file!");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Cant read file!");
        println!("File contents: \n\n{}", contents); */

        /* let args: Vec<String> = env::args().collect();

        for argument in args.iter() {
            println!("{}", argument);
        }

        println!("{}", args[1]); */

        /* let mut file = File::create("output.txt").expect("Cant create file!");
        file.write_all(b"Welcome").expect("Cant write file!"); */

        /* let person = Person {
            name: String::from("Jm Sar"),
            age: 0
        };

        println!("Can {} speak? {}", person.name, person.can_speak()); */

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

        /* let random_number = rand::thread_rng().gen_range(2..11); //1-10
        println!("Random number: {}", random_number);

        let random_bool = rand::thread_rng().gen_bool(2.0/3.0);
        println!("Random boolean: {}", random_bool); */

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

        /* jmsar::print_message(); */

        /*  let re = Regex::new(r"(\w{5})").unwrap();
           let text = "jmsar";
           //println!("{}", re.is_match(text));
           match re.captures(text){
               Some(caps) => println!("Found match: {}", &caps[0]),
               None => println!("Couldnt find a match")
           }
        */
        /* jmsar2::print_message();
        jmsar2::water::print_message(); */

        /* let name = String::from("Jmsar");

        println!("{}", match name.chars().nth(1){
           Some(c) => c.to_string(),
           None => "No character at index 8!".to_string()
        }) */

        /* println!("Occupation is {}", match get_occupation("Jmsar"){
           Some(o) => o,
           None => "No occuption found"
        }); */

        /* let d = Day::Saturday;
        println!("Is d a weekday? {}", d.is_weekday());
         */

        /* let mut cmd = Command::new("python");
        cmd.arg("jmsar.py");

        match cmd.output(){
          Ok(o) => {
              unsafe{
                  println!("{}", String::from_utf8_unchecked(o.stdout));
              }

          }
          Err(e) => {
              println!("{}", e);
          }
        } */

        /* match reqwest::get("https://weatherapi-com.p.rapidapi.com/current.json?q=Madrid"){
           Ok(mut response) => {
               if response.status() == reqwest::StatusCode::Ok {
                   match response.text(){
                       Ok(text) => println!("Response text: {}", text),
                       Error(_) => println!("Error text")
                   }
               } else {
                   println!("Response was not 200 ok")
               }
           }
           Err(_) => println!("Error")
         }
        */

        /* let response_text = reqwest::get("https://weatherapi-com.p.rapidapi.com/current.json?q=Madrid")
        .await.expect("Couldnt make request")
        .text().await.expect("Could read the response text");
         println!("{}", response_text); */

        let json_str = r#"
    {
        "name":"jmsar",
        "age": 37,
        "is_male": true
    }
  "#;

        let res = serde_json::from_str(json_str);

        if res.is_ok() {
            let p: Person = res.unwrap();
            println!("Name is {}", p.name);
        } else {
            println!("didnt work");
        }
    }

    /* enum Direction {
        Up,
        Down,
        Left,
        Right
    } */

    // const MAXIMUN_NUMBER:u8 = 20;

    /* fn print_numbers_to(num: u32){
        for n in 1..num {
            if is_even(n){
                println!("{}", n);
            }
        }
    }

    fn is_even(num: u32) -> bool {
        return num % 2 == 0;
    } */

    /* struct Color {
        red: u8, //u8: 0-255
        green: u8,
        blue: u8
    } */

    /* struct Color(u8, u8, u8);

    fn print_color(c: &Color){
        println!("{} {} {}", c.0, c.1, c.2);
    } */

    /* struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle {
        fn print_description(&self){
            println!("Width: {} Height: {}", self.width, self.height);
        }

        fn is_square(&self) -> bool {
            return self.width == self.height
        }
    } */

    /* struct Person {
        name: String,
        age: u8
    }

    trait HasVoiceBox {
        fn speak(&self);

        fn can_speak(&self) -> bool;
    }

    impl HasVoiceBox for Person {
        fn speak(&self){
            println!("Hellooooo!!!")
        }

        fn can_speak(&self) -> bool {
            if self.age > 0 {
                return true;
            } return false;
        }
    } */

    /*

    impl ToString for Person {
        fn to_string(&self) -> String {
            return format!("My name is {} and I am {}", self.name, self.age);
        }
    } */

    /* fn get_occupation(name: &str) -> Option<&str> {
       match name {
           "Jmsar" => Some("Software Developer"),
           "Michael" => Some("Dentist"),
           _ => None
       }
    } */

    /*  enum Day {
       Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
    }

    impl Day {
       fn is_weekday(&self) -> bool {
           match self {
               &Day::Saturday | &Day::Sunday => return false,
               _ => true
           }
       }
    } */

    /*  fn give_two() -> i32 {
       2
    }

    #[cfg(test)] //doest compile
    mod jmsar_tests {
       #[test]
       #[should_panic]
       fn test_basic(){
           assert!(1 == 1);
           panic!("Oh no!");
       }

       #[test]
       #[ignore]
       fn test_equals(){
           assert_eq!(super::give_two(), 1 + 1);
           assert_ne!(super::give_two(), 1 + 3);
       }

       #[test]
       #[should_panic]
       fn test_structs(){
           let r = super::Rectangle {
               width: 50,
               height: 25
           };

           assert!(r.is_square());
       }
    } */
}
