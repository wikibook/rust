fn main() {
    // 문자열로 이루어진 배열
    let array = [
        "Apple".to_string(), 
        "Banana".to_string(), 
        "Mango".to_string(), 
        "Tomato".to_string()
    ];
    // 배열을 반복해 화면에 출력
    for a in array.iter() { // ← 수정
        println!("{}", a);
    }
    println!("len={}", array.len()); // ← OK
}
