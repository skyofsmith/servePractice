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

impl GetName for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl GetAge for Teacher {
    fn get_age(&self) -> u32 {
        self.age
    }
}
// error
/* 
fn produce_item_with_age() -> impl GetAge {
    let is = true;
    if is {
        Student {
            name: String::from("xiaoming"),
            age: 16,
        }
    } else {
        Teacher {
            name: String::from("xiaohuang"),
            age: 30
        }
    }

} */
struct PeopleMatchInformation<T, U> {
    master: T,
    student: U,
}

impl<T: GetName+GetAge, U:GetName+GetAge> PeopleMatchInformation<T, U> {
    fn print_all_information(&self) {
        println!("master name = {}", self.master.get_name());
        println!("master age = {}", self.master.get_age());
        println!("student name = {}", self.student.get_name());
        println!("student age = {}", self.student.get_age());
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

    // let _s = produce_item_with_age();

    let p_s = Student {
        name: String::from("xiaoming"),
        age: 18
    };
    let p_t = Teacher {
        name: String::from("xiaohuang"),
        age: 30,
        subject: String::from("math")
    };
    let p = PeopleMatchInformation {
        master: p_t,
        student: p_s
    };
    p.print_all_information();
}
