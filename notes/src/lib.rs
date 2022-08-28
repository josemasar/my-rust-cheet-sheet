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

        //Read file

        /* let mut file = File::open("info.txt").expect("Cant open file!");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Cant read file!");
        println!("File contents: \n\n{}", contents); */

        //Write file

        /* let mut file = File::create("output.txt").expect("Cant create file!");
        file.write_all(b"Welcome").expect("Cant write file!"); */

        /* let person = Person {
            name: String::from("Jm Sar"),
            age: 0
        };

        println!("Can {} speak? {}", person.name, person.can_speak()); */

        //Threads
        /* let random_number = rand::thread_rng().gen_range(2..11); //1-10
        println!("Random number: {}", random_number);

        let random_bool = rand::thread_rng().gen_bool(2.0/3.0);
        println!("Random boolean: {}", random_bool); */

        //Regular expressions

        /*  let re = Regex::new(r"(\w{5})").unwrap();
           let text = "jmsar";
           //println!("{}", re.is_match(text));
           match re.captures(text){
               Some(caps) => println!("Found match: {}", &caps[0]),
               None => println!("Couldnt find a match")
           }
        */

        //Read from python file

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

        //HTTP Request

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

        //Serialize Json

        /* let json_str = r#"
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
              } */
    }
}
