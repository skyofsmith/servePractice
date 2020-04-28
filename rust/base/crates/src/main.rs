mod factory {
    pub mod produce_refrigerator {
        pub fn produce_re() {
            println!("produce re!");
        }
    }

    mod produce_washing_machine {
        fn produce_washing_machine() {
            println!("produce washing machine!");
        }
    }
}

fn main() {
    factory::produce_refrigerator::produce_re();
    // factory::produce_washing_machine::produce_washing_machine();
    println!("Hello, world!");
}
