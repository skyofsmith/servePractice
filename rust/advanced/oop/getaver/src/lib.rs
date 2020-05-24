pub struct AverCollect {
    list: Vec<i32>,
    aver: f64,
}

impl AverCollect {
    pub fn new() -> AverCollect {
        AverCollect {
            list: vec![],
            aver: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(v) => {
                self.update_average();
                Some(v)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.aver
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.aver = total as f64 / self.list.len() as f64;
    }
}

// Rust里面没有继承的概念，可以通过trait来进行行为共享
// trait A {
// 	fn sum() {
// 		//todo
// 	}
// }

// struct XXX {
// }

// impl A for XXX {
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
