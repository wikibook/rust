fn main() {
    // 명령줄 인수를 Vec<String>으로 취득
    let args:Vec<String> = std::env::args().collect();
    println!("{:?}", args);
}