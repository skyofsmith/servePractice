use mylib::factory::produce_refrigerator;
// use mylib::factory::produce_refrigerator::produce_re; // 不推荐使用
use mylib::factory::produce_washing_machine as A;

// use mylib::factory::*; // 导入所有包

fn main() {
    // 绝对路径
    mylib::factory::produce_refrigerator::produce_re();

    produce_refrigerator::produce_re();
    A::produce_re();

    // produce_re();  // 不推荐使用
}
