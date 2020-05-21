use std::thread;
use std::time::Duration;

fn main() {
    {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            // sleep(10);
            println!("v: {:?}", v);
        });

        // drop(v);
        // println!("v: {:?}", v);  // error v moved
        

        handle.join().unwrap();
    }
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("number {} in spawn thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("number {} in main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();

        println!("++++++++++++++++");
        for i in 1..5 {
            println!("number {} in main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

    }
}
