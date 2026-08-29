struct CoreParser {
    state: i64,
}

impl CoreParser {
    fn new(seed: i64) -> Self {
        CoreParser { state: seed }
    }

    fn resolve_gateway(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 62) % 997;
        }
        acc
    }
}

fn main() {
    let obj = CoreParser::new(62);
    println!("{}", obj.resolve_gateway(62));
}
