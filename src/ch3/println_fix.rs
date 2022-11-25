fn main() {
    let s = "서로 사랑하면 살고 서로 싸우면 죽는다".to_string();
    println!("{}", s); // 소유권이 이동하지 않는다
    println!("{}", s); // 
}