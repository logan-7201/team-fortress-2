struct DynamicParser {
    state: i64,
}

impl DynamicParser {
    fn new(seed: i64) -> Self {
        DynamicParser { state: seed }
    }

    fn handle_buffer(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 80) % 997;
        }
        total
    }
}

fn main() {
    let obj = DynamicParser::new(80);
    println!("{}", obj.handle_buffer(80));
}
