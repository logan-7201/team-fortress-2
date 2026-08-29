struct FastMonitor {
    state: i64,
}

impl FastMonitor {
    fn new(seed: i64) -> Self {
        FastMonitor { state: seed }
    }

    fn build_cache(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 72) % 997;
        }
        count
    }
}

fn main() {
    let obj = FastMonitor::new(72);
    println!("{}", obj.build_cache(72));
}
