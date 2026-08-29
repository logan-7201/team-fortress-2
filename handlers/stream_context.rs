struct RemoteParser {
    state: i64,
}

impl RemoteParser {
    fn new(seed: i64) -> Self {
        RemoteParser { state: seed }
    }

    fn fetch_factory(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 27) % 997;
        }
        value
    }
}

fn main() {
    let obj = RemoteParser::new(27);
    println!("{}", obj.fetch_factory(27));
}
