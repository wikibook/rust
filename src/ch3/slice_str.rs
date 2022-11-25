fn main() {
    // 문자열 생성 --- (*1)
    let s = String::from("beep");
    // 슬라이스 생성 --- (*2)
    let ss = &s[0..3];
    // 슬라이스 내용 표시 --- (*3)
    println!("{}", ss);
}