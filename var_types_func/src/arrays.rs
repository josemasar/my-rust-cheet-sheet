#[allow(dead_code)]

pub mod arrays {
    pub fn learn_arrays() {
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
    }
}
