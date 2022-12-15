use std::collections::HashMap;
fn main() {
    // 우리말 월 이름 목록 --- (*1)
    let months = ["해오름달", "시샘달", "꽃내음달", "잎새달", "푸른달", "누리달", 
        "빗방울달", "타오름달", "거둠달", "온누리달", "눈마중달", "매듭달"];
    
    // HashMap 초기화 --- (*2)
    let mut months_map: HashMap<&str, usize> = HashMap::new();
    // 월의 이름을 HashMap에 추가 --- (*3)
    for (i, v) in months.iter().enumerate() {
        months_map.insert(v, i+1);
    }
    // 요소를 선택해 표시 --- (*4)
    println!("누리달 = {}월", months_map["누리달"]);
    println!("온누리달 = {}월", months_map["온누리달"]);
    println!("매듭달 = {}월", months_map["매듭달"]);
}

