pub mod iterate {
    pub fn learn_loops() {
        // loop: Repeat, unless a manual stop occurs.
        // while: Repeat while a condition remains true.
        // for: Repeat for all values in a collection.
        loop {
            println!("We loop forever!");
            break;
        }

        let mut counter = 0;
        let finish_count = loop {
            counter += 1;
            println!("I´m counting {}", counter);
            if counter > 100 {
                break counter;
            }
        };

        println!("{:?}", finish_count);

        let mut counter_while: i32 = 1;

        while counter_while < 5 {
            println!("We loop a while...{}", counter_while);
            counter_while = counter_while + 1;
        }

        let big_birds = ["ostrich", "peacock", "stork"];
        for bird in big_birds.iter() {
            println!("The {} is a big bird.", bird);

            for number in 0..5 {
                println!("{}", number * 2);
            }
        }
    }
}
