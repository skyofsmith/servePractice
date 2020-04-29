// use mylib::factory::produce_refrigerator;
// use mylib::factory::produce_refrigerator::produce_re; // 不推荐使用
// use mylib::factory::produce_washing_machine as A;

use mylib::factory::*; // 导入所有包

mod mod_a {
    #[derive(Debug)]
    pub struct A {
        pub number: i32,
        name: String,
    }
    impl A {
        pub fn new_a() -> A {
            A {
                number: 1,
                name: String::from("A"),
            }
        }
        pub fn print_a(&self) {
            println!("number: {}, name: {}", self.number, self.name)
        }
    }

    pub mod mod_b {
        pub fn print_b() {
            println!("B");
        }

        pub mod mod_c {
            pub fn print_c() {
                println!("C");
                super::print_b();
            }
        }
    }

}

use mod_a::A;

fn main() {
    // 绝对路径
    mylib::factory::produce_refrigerator::produce_re();

    produce_refrigerator::produce_re();
    produce_washing_machine::produce_re();

    // produce_re();  // 不推荐使用

    let a = A::new_a();
    a.print_a();

    let number = a.number;
    println!("number = {}", number);

    // let name = a.name; // private field
    // println!("name = {}", name);

    mod_a::mod_b::mod_c::print_c();
}
