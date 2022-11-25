// Counter 구조체를 정의 --- (*1)
struct Counter {
    value: i64,
}
impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }
    fn inc(&mut self) {
        self.value += 1;
        println!("value={}", self.value);
    }
}

// Counter 구조체를 인수로 하는 함수 --- (*2)
fn count(counter: Option<&mut Counter>) {
    match counter {
        None => return,
        Some(c) => c.inc(),
    };
}

fn main() {
    // Counter 구조체를 인수로 해 count 함수를 호출--- (*3)
    let mut a = Counter::new();
    count(Some(&mut a));
    count(Some(&mut a));
    // None 객체를 인수로 해 호출 --- (*4)
    let a = None;
    count(a);
}
