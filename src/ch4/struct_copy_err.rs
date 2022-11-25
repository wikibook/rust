#[derive(Clone)]
struct Person {
    name: String,
    age: i32,
}
impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self { name: name.to_string(), age }
    }
}

fn main() {
    // Alex 생성
    let alex = Person::new("Alex", 18);
    // Betty는 Alex를 복사해 이름만 변경하고자 함 --- (*1)
    let mut betty = alex.clone();
    betty.name = String::from("Betty");
    // Alex와 Betty를 출력
    println!("{},{}", alex.name, alex.age); // ← 에러 --- (*2)
    println!("{},{}", betty.name, betty.age);
}
