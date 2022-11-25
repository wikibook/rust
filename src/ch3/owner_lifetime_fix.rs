// 메시지 생성 함수
fn gen_message() -> String {
    let msg = String::from("실수할 줄 아는 사람이 아름답다");
    return msg; // 소유권이 함수의 반환 값으로 이동 --- (*1)
}
fn main() {
    let m = gen_message(); // 소유권은 m으로 이동 --- (*2)
    println!("{}", m); // ok
}