use mylib::factory::produce_refrigerator;
// use mylib::factory::produce_refrigerator::produce_re; // 不推荐使用
use mylib::factory::produce_washing_machine as A;

// use mylib::factory::*; // 导入所有包

mod modA {
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
}

fn main() {
    // 绝对路径
    mylib::factory::produce_refrigerator::produce_re();

    produce_refrigerator::produce_re();
    A::produce_re();

    // produce_re();  // 不推荐使用

    let a = modA::A::new_a();
    a.print_a();
}
