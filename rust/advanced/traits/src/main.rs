//1、trait用于定义与其他类型共享的功能，类似与其他语言中的接口
//（1）可以通过trait以抽象的方式定义共享的行为
//（2）可以使用trait bounds指定泛型是任何拥有制定行为的类型
//2、定义trait
pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

trait SchoolName {
    fn get_school_name(&self) -> String {
        String::from("HongXingSchool")
    }
}
//3、实现trait
pub struct Student {
    pub name: String,
    pub age: u32,
}
impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}
impl SchoolName for Student {}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}
impl GetInformation for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}
impl SchoolName for Teacher {
    fn get_school_name(&self) -> String {
        String::from("GuangMingSchool")
    }
}
//4、默认实现：可以在定义trait的时候提供默认的行为，trait的类型可以使用默认的行为。
//5、trait作为参数
//直接作为参数写法
fn print_information(item: impl GetInformation) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

//trait_bound语法
trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}

//写法1:
fn print_informations<T: GetName+GetAge>(item: T) {
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

//写法2:
fn print_informations2<T>(item: T)
    where T: GetName+GetAge
{
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}

impl GetName for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl GetAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn produce_item_with_age() -> impl GetAge {
    #[derive(Debug)]
    Student {
        name: String::from("xiaoming"),
        age: 16,
    }
}

fn main() {
    let s = Student {
        name: "xiaoming".to_string(),
        age: 10
    };
    let t = Teacher {
        name: "xiaohuang".to_string(),
        age: 30,
        subject: "math".to_string()
    };

    let ssn = s.get_school_name();
    let tsn = t.get_school_name();
    println!("student school name = {}", ssn);
    println!("teacher school name = {}", tsn);
    print_information(s);
    print_information(t);

    let stu = Student {
        name: "XiaoZhang".to_string(), 
        age: 10
    };
    let stu2 = Student {
        name: "XiaoWang".to_string(), 
        age: 30
    };
    print_informations(stu);
    print_informations2(stu2);

    let _s = produce_item_with_age();
}
