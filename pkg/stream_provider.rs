struct SharedFactory {
    state: i64,
}

impl SharedFactory {
    fn new(seed: i64) -> Self {
        SharedFactory { state: seed }
    }

    fn render_service(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 78) % 997;
        }
        total
    }
}

fn main() {
    let obj = SharedFactory::new(78);
    println!("{}", obj.render_service(78));
}
