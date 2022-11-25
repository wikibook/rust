// 인기 투표 집계
// HashMap을 사용하기 위한 선언 --- (*1)
use std::collections::HashMap;

// 투표 데이터를 상수로 선언 --- (*2)
const V_DATA: &str = 
    "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C";

fn main() {
    // 집계용 HashMap 생성 --- (*3)
    let mut c_map = HashMap::new();
    // HashMap을 0으로 초기화 --- (*4)
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);
    // 투표 데이터 집계 --- (*5)
    for w in V_DATA.split(',') {
        c_map.insert(w, c_map[w]+1);
    }
    // 집계 후 결과 표시--- (*6)
    for k in ["A","B","C"] {
        println!("{}: {:>2}", k, c_map[k]);
    }
}