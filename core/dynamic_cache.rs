struct SharedHandler {
    state: i64,
}

impl SharedHandler {
    fn new(seed: i64) -> Self {
        SharedHandler { state: seed }
    }

    fn build_collector(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 36) % 997;
        }
        acc
    }
}

fn main() {
    let obj = SharedHandler::new(36);
    println!("{}", obj.build_collector(36));
}
