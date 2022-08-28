#[allow(dead_code)]

pub mod structs {
    pub fn learn_structs() {
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
    }
}
