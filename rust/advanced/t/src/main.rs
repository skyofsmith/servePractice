//1、泛型是具体类型或者其他属性的抽象替代，用于减少代码重复。
//2、在函数定义中使用泛型
//3、在结构体中使用泛型
//4、枚举中的泛型
//5、方法中的泛型
//6、总结：使用泛型并不会造成程序性能上的损失。rust通过在编译时进行泛型代码的单态化来保证效率。单态化时通过填充编译时使用的具体类型，将通用代码转化为特定代码的过程。
//---------------无泛型-------------------------
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
//---------------无泛型-------------------------

//--------------使用泛型-------------------------
fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}
//--------------使用泛型-------------------------
fn main() {
    let number_list = vec![1, 2, 23, 34, 8, 100];
    let max_number = largest_i32(&number_list);
    println!("max_number = {}", max_number);
    let max_num = largest(&number_list);
    println!("max_num = {}", max_num);

    let char_list = vec!['a', 'y', 'b'];
    let max_char = largest_char(&char_list);
    println!("max_char = {}", max_char);
    let max_c = largest(&char_list);
    println!("max_c = {}", max_c);
}
