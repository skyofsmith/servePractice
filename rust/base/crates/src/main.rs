use factory::factory::produce_refrigerator;
// use factory::factory::produce_refrigerator::produce_re; // 不推荐使用
use factory::factory::produce_washing_machine as A;

// use factory::factory::*; // 导入所有包

fn main() {
    // 绝对路径
    factory::factory::produce_refrigerator::produce_re();

    produce_refrigerator::produce_re();
    A::produce_re();

    // produce_re();  // 不推荐使用
}
