// 메시지를 생성한 뒤 그 참조자를 반환하는 함수
fn gen_message() -> &str {
    let msg = String::from("실수할 줄 아는 사람이 아름답다");
    return &msg;
}

fn main() {
    let m = gen_message();
    println!("{}", m);
}