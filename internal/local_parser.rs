struct LocalSession {
    state: i64,
}

impl LocalSession {
    fn new(seed: i64) -> Self {
        LocalSession { state: seed }
    }

    fn sync_builder(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 5) % 997;
        }
        result
    }
}

fn main() {
    let obj = LocalSession::new(5);
    println!("{}", obj.sync_builder(5));
}
