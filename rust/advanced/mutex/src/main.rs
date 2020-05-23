//1、通道类似于单所有权的方式，值传递到通道后，发送者就无法再使用这个值；
//2、共享内存类似于多所有权，即多个线程可以同时访问相同的内存位置。

//互斥器：mutex
//1、任意时刻，只允许一个线程来访问某些数据;
//2、互斥器使用时，需要先获取到锁，使用后需要释放锁。
//Mutex<T>
//Mutex<T>是一个智能指针，lock调用返回一个叫做MutexGuard的智能指针
//内部提供了drop方法，实现当MutexGuard离开作用域时自动释放锁。

use std::sync::{Mutex, Arc};
use std::thread;
// use std::rc::Rc;

fn main() {
    {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }   //离开作用域时，自动释放
        println!("m = {:?}", m);
    }


    //RefCell\Rc\Box

    //RefCell<T>/Rc<T> 与Mutex<T>/Arc<T>
    //1、Mutex<T>提供内部可变性，类似于RefCell
    //2、RefCell<T>/Rc<T>是非线程安全的， Mutex<T>/Arc<T>是线程安全的

    {
        //Rc<T> 不是线程安全的
        // let counter = Mutex::new(0);
        // let counter = Rc::new(Mutex::new(0));
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..10 {
            let cnt = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = cnt.lock().unwrap();
                *num += 1;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("counter = {:?}", *counter.lock().unwrap());
    }
}
