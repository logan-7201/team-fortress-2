struct DynamicManager {
    state: i64,
}

impl DynamicManager {
    fn new(seed: i64) -> Self {
        DynamicManager { state: seed }
    }

    fn fetch_resolver(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 45) % 997;
        }
        total
    }
}

fn main() {
    let obj = DynamicManager::new(45);
    println!("{}", obj.fetch_resolver(45));
}
