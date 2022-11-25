// 구조체 Person을 정의
struct Person {
    name: String,
    age: i32,
}
// Person의 메서드를 정의
impl Person {
    // Person을 생성하는 함수를 정의 --- (*1)
    fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }
}

fn main() {
    // 연관 함수 new를 이용해 객체 생성
    let yang = Person::new("양현".to_string(), 18);
    // 객체 출력
    println!("{}씨는 마음만은 {}살", yang.name, yang.age);
}
