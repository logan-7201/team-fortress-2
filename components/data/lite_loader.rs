struct AsyncSession {
    state: i64,
}

impl AsyncSession {
    fn new(seed: i64) -> Self {
        AsyncSession { state: seed }
    }

    fn load_client(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 63) % 997;
        }
        acc
    }
}

fn main() {
    let obj = AsyncSession::new(63);
    println!("{}", obj.load_client(63));
}
