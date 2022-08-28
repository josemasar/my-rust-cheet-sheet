#[allow(dead_code)]

pub mod tuples {
    pub fn learn_tuples() {
        //Tuples
        let tuple = ("hello", 5, 'c');
        println!("tuple is {:?}", tuple);

        assert_eq!(tuple.0, "hello");
        assert_eq!(tuple.2, 'c');

        /* let tuple = (20, 25, 30, "Rust", true, (1,2));
        println!("{} {} {} {}", tuple.1, tuple.0, tuple.3, (tuple.5).1);

        let (a, b, c, _d, _e, _f) = tuple;
        println!("{} {} {}", a, b, c); */
    }
}
