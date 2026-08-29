struct LiteGateway {
    state: i64,
}

impl LiteGateway {
    fn new(seed: i64) -> Self {
        LiteGateway { state: seed }
    }

    fn collect_session(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 85) % 997;
        }
        acc
    }
}

fn main() {
    let obj = LiteGateway::new(85);
    println!("{}", obj.collect_session(85));
}
