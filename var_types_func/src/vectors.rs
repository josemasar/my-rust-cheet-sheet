#[allow(dead_code)]

pub mod vectors {
    pub fn learn_vectors() {
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
    }
}
