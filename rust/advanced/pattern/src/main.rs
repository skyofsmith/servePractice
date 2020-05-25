//1、模式是Rust中特殊的语法，模式用来匹配值的结构。
//
//2、模式由如下内容组成：
//（1）字面值
//（2）解构的数组、枚举、结构体或者元组
//（3）变量
//（4）通配符
//（5）占位符

fn main() {
    {
        //1、match
        //match VALUE {
        //    PATTERN => EXPRESSION,
        //    PATTERN => EXPRESSION,
        //    PATTERN => EXPRESSION,
        //}
        //必须匹配完所有的情况
        let a = 1;    
        match a {
            0 => println!("Zero"),
            1 => println!("One"),
            _ => println!("other"),
        };
    }

    {
        //if let
        let color: Option<&str> = None;
        let is_ok = true;
        let age: Result<u8, _> = "33".parse();

        if let Some(c) = color {
            println!("color: {}", c);
        } else if is_ok {
            println!("is ok");
        } else if let Ok(a) = age {
            if a > 30 {
                println!("oh, mature man");
            } else {
                println!("oh, young man");
            }
        } else {
            println!("in else");
        }
    }

    {
        //while let
        //只要模式匹配就一直执行while循环
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("top = {}", top);
        }//只要匹配Some(value),就会一直循环
    }

    {
        //for 
        //在for循环中，模式是直接跟随for关键字的值，例如 for x in y，x就是对应的模式
        let v = vec!['a', 'b', 'c'];
        for v in v.iter() {
            println!("{}", v);
        }
        for (index, value) in v.iter().enumerate() {
            println!("index: {}, value: {}", index, value);
        }
        //此处的模式是(index, value)
    }

    {
        //let 
        //let PATTERN = EXPRESSION
        let (x, y, z) = (1, 2, 3); //(1, 2, 3)会匹配(x, y, z)，将1绑定到x，2绑定到y，3绑定到z
        println!("{}, {}, {}", x, y, z);

        let (x, .., z) = (1, 2, 3);
        println!("{}, {}", x, z);
    }

    {
        //函数
        //函数的参数也是模式
        fn print_point(&(x, y): &(i32, i32)) {
            println!("x: {}, y: {}", x, y);
        } 
        let p = (3, 5);
        print_point(&p);
        //&(3, 5) 匹配模式 &(x, y)
        //模式在使用它的地方并不都是相同的，模式存在不可反驳的和可反驳的
    }

    {
        //1、模式有两种：refutable（可反驳的）和 irrefutable（不可反驳的）。能匹配任何传递的可能值的模式被称为是不可反驳的。对值进行匹配可能会失败的模式被称为可反驳的。
        //2、只能接受不可反驳模式的有：函数、let语句、for循环。原因：因为通过不匹配的值程序无法进行有意义的工作。
        //3、if let和while let表达式被限制为只能接受可反驳的模式，因为它们的定义就是为了处理有可能失败的条件。
        let a: Option<i32> = Some(5); //匹配Some(value), None
        let b: Option<i32> = None; //匹配Some(value), None
        // let Some(x) = a;
        // let Some(x) = b;

        if let Some(v) = a {
            println!("v {}", v);
        }
        // warn occur
        if let v = 5 {
            println!("v {}", v);
        }
    
    }
}
