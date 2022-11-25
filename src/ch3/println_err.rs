fn main() {
    let s = "서로 사랑하면 살고 서로 싸우면 죽는다".to_string();
    echo(s); // ← 소유권이 이동한다
    println!("{}", s);
}
// println!을 모방한 함수
fn echo(s: String) {
    println!("{}", s);
}