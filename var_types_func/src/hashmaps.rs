#[allow(dead_code)]

pub mod hashmaps {
    use std::collections::HashMap;
    pub fn learn_hashmaps() {
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
