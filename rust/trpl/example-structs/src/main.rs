fn main() {
    test_rebuild_by_turpl();
    test_rebuild_by_structs();
}

fn test_rebuild_by_turpl() {

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn test_rebuild_by_structs() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:#?}", rect1);
}
