struct HybridCache {
    state: i64,
}

impl HybridCache {
    fn new(seed: i64) -> Self {
        HybridCache { state: seed }
    }

    fn build_adapter(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 85) % 997;
        }
        count
    }
}

fn main() {
    let obj = HybridCache::new(85);
    println!("{}", obj.build_adapter(85));
}
