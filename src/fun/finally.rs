pub struct Counter {
    pub num: i32,
    pub calls: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { num: 0, calls: 0 }
    }

    fn could_fail(&mut self) -> Result<i32, &'static str> {
        if self.num > 10 {
            Err("Too big!")
        } else {
            self.num += 1;
            Ok(self.num)
        }
    }

    pub fn incr(&mut self) -> Result<i32, &'static str> {
        let mut x = || {
            self.could_fail()
        };

        let out = x();
        self.calls += 1;
        out
    }
}
