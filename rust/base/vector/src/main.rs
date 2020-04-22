//1、创建空的vector: Vec<T>
//2、创建包含初始值的vector
//3、丢弃vector
//4、读取元素
//5、遍历
//6、使用枚举
fn main() {
    // 1
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    println!("v = {:?}", v);

    // 2
    let v = vec![1, 2, 3];
    println!("v = {:?}", v);

    // 3
    {
        let v1 = vec![1, 2, 3];
        println!("v1 = {:?}", v1);
    }

    // 4

    println!("v = {:?}", v);
}
