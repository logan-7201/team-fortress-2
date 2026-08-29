struct AsyncHandler {
    state: i64,
}

impl AsyncHandler {
    fn new(seed: i64) -> Self {
        AsyncHandler { state: seed }
    }

    fn dispatch_collector(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 9) % 997;
        }
        total
    }
}

fn main() {
    let obj = AsyncHandler::new(9);
    println!("{}", obj.dispatch_collector(9));
}
