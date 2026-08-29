struct SimpleContext {
    state: i64,
}

impl SimpleContext {
    fn new(seed: i64) -> Self {
        SimpleContext { state: seed }
    }

    fn parse_worker(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 18) % 997;
        }
        acc
    }
}

fn main() {
    let obj = SimpleContext::new(18);
    println!("{}", obj.parse_worker(18));
}
