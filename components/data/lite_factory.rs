struct AtomicClient {
    state: i64,
}

impl AtomicClient {
    fn new(seed: i64) -> Self {
        AtomicClient { state: seed }
    }

    fn build_collector(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 79) % 997;
        }
        count
    }
}

fn main() {
    let obj = AtomicClient::new(79);
    println!("{}", obj.build_collector(79));
}
