pub mod extern_sub_module {
    pub struct Sqoiner {
        pub number: i32,
    }

    pub trait SqoinRules {
        fn shownumber(sqoiner: &Sqoiner);
    }

    pub fn print_sqoiner(sqoiner: &Sqoiner) -> () {
        println!(" Number Is {}", sqoiner.number);
    }
}
