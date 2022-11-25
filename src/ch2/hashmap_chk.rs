use std::collections::HashMap;
fn main() {
    // HashMap을 생성해 초기화
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);
    // 키가 존재하지 않는지 확인
    if map.get("D") == None {
        println!("D는 존재하지 않음")
    } else {
        // 값을 표시
        println!("D={}", map["D"]);
    }
}