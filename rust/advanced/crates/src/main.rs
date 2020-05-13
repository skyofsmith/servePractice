//crate的发布与撤回
//（1）创建Crates.io账号：通过Github账户注册，并通过cargo login 来登陆
//（2）发布前需要在Cargo.toml中增加描述：
// [package]
// name = "package_name"
// version = "0.1.0"
// license = "MIT"
// authors = ["sam"]
// description = "sam's lib"
//运行cargo publish来发布。
//（3）撤回指定版本
//cargo yank --vers 0.1.0

fn main() {
    println!("Hello, world!");
}
