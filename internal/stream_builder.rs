struct SmartFactory {
    state: i64,
}

impl SmartFactory {
    fn new(seed: i64) -> Self {
        SmartFactory { state: seed }
    }

    fn compute_context(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 22) % 997;
        }
        count
    }
}

fn main() {
    let obj = SmartFactory::new(22);
    println!("{}", obj.compute_context(22));
}
