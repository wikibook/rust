// 에러가 발생하는 프로그램
use std::collections::HashMap;
fn main() {
    // HashMap을 생성해 초기화
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);
    // 존재하지 않는 키를 취득
    let d = map["D"];
    println!("{}", d);
}