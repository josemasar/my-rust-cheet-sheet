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

        /*
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

        // const MAXIMUN_NUMBER:u8 = 20;

        /* for n in 1..MAXIMUN_NUMBER {
            println!("{}", n);
        } */

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

        /* print_numbers_to(10);

        if is_even(6){
            println!("is even");
        } */
    }
}
