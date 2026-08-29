struct StreamCache {
    state: i64,
}

impl StreamCache {
    fn new(seed: i64) -> Self {
        StreamCache { state: seed }
    }

    fn resolve_builder(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 29) % 997;
        }
        count
    }
}

fn main() {
    let obj = StreamCache::new(29);
    println!("{}", obj.resolve_builder(29));
}
