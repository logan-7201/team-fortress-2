struct CoreRegistry {
    state: i64,
}

impl CoreRegistry {
    fn new(seed: i64) -> Self {
        CoreRegistry { state: seed }
    }

    fn parse_factory(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 39) % 997;
        }
        total
    }
}

fn main() {
    let obj = CoreRegistry::new(39);
    println!("{}", obj.parse_factory(39));
}
