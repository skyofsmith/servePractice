
fn main() {
    {
        //1、匹配字面值
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            _ => println!("xx"),
        };
    }
    {
        //2、匹配命名变量
        let x = Some(5);
        let y = 10; //位置1
        match x {
            Some(50) => println!("50"),
            Some(y) => println!("value = {}", y),   //此处的y不是位置1的y
            _ => println!("other"),
        };
        println!("x = {:?}, y = {}", x, y); //此处的y是位置1的y
    }
    {
        //3、多个模式
        let x = 1;
        match x {
            1|2 => println!("1 or 2"),  //|表示是或，匹配1或者2
            3 => println!("3"),
            _ => println!("xx"),
        }
    }
    {
        //4、通过..匹配
        let x = 5;
        match x {
            1..=5 => println!("1 to 5"),    // 1|2|3|4|5 => println!("1 to 5"),
            _ => println!("xx"),
        };

        let x = 'c';
        match x {
            'a'..='j' => println!("1"),
            'k'..='z' => println!("2"),
            _ => println!("other"),
        };
    }

    //解构并分解值
    //解构元祖、结构体、枚举、引用
    {
        //解构结构体
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 1, y: 2};
        let Point { x: a, y: b } = p;
        assert_eq!(1, a);
        assert_eq!(2, b);

        let Point { x, y } = p;
        assert_eq!(1, x);
        assert_eq!(2, y);

        let p = Point { x: 1, y: 0};
        match p {
            Point { x, y: 0 } => println!("x axis, x = {}", x),
            Point { x: 0, y } => println!("y axis, y = {}", y),
            Point { x, y } => println!("other, x = {}, y = {}", x, y),
        };
    }
    {
        //结构枚举
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => {
                println!("quit");
            },
            Message::Move { x, y } => {
                println!("move, x: {}, y: {}", x, y);
            },
            Message::Write(text) => println!("write msg: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("color, r: {}, g: {}, b: {}", r, g, b);
            },
        }
    }
    {
        //结构嵌套枚举
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("rgb color, r: {}, g: {}, b: {}", r, g, b);
            },
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("hsv color, h: {}, s: {}, v: {}", h, s, v);
            },
            _ => ()
        }
    }
    {
        //解构元祖
        struct Point {
            x: i32,
            y: i32,
        }

        let ((a, b), Point{ x, y }) = ((1, 2), Point{ x: 3, y: 4});
        println!("a: {}, b: {}, x: {}, y: {}", a, b, x, y);
    }

    //忽略模式中的值
    {
        fn foo(_: i32, y: i32) {
            println!("y = {}", y);
        }

        trait A {
            fn bar(x: i32, y: i32);
        }

        struct B  {}
        impl A for B {
            fn bar(_: i32, y: i32) {
                println!("y = {}", y);
            }
        }
        foo(1, 2);

        let numbers = (1, 2, 3, 4);
        match numbers {
            (one, _, three, _) => {
                println!("one: {}, three: {}", one, three);
            },
        }
    }
    {
        let _x = 5;
        let _y = 5;
        println!("_x: {}", _x);

        let s = Some(String::from("hello"));
        if let Some(_c) = s {
            println!("found a string");
        }
        // println!("s: {:?}", s); // error


        let s = Some(String::from("hello"));
        if let Some(_) = s {
            println!("found a string");
        }
        println!("s: {:?}", s); // no error
    }

    {
        let numbers = (1,2,3,4,5,6,7);
        match numbers {
            (first, .., last) => {
                println!("first: {}, last: {}", first, last);
            },
        };
        // match numbers {
        //     (.., second, ..) => { // error occur
        //         println!("second: {}", second);
        //     },
        // };
    }

    {
        //7、匹配守卫提供额外的条件
        //匹配守卫是一个指定于match分之模式之后的额外的if条件，必须满足才能选择此分支。
        let num = Some(5);
        match num {
            Some(x) if x < 5 => println!("< 5"),
            Some(x) => println!("x: {}", x),
            None => (),
        };

        let y = 10; //位置1
        match num {
            Some(x) if x == y => println!("num == y"), //此处的y是位置1处的y
            Some(x) => println!("x: {}", x),
            None => (),
        };

        let x = 4;
        let y = false;
        match x {
            4|5|6 if y => println!("1"), //4|5|6 if y  a: 4|5|(6 if y)   b: ((4|5|6) if y)(等价于此种)
            _ => println!("2"),
        };
    }
}
