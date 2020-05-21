use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // sleep(10);
        println!("v: {:?}", v);
    });

    // drop(v);
    // println!("v: {:?}", v);  // error v moved
    

    handle.join().unwrap();
}
