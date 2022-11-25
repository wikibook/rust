// 곱셈 함수 정의
fn multiplication(a: i64, b: i64) -> i64 {
    a * b
}

fn main() {
    // 함수 호출
    let ex1 = multiplication(3, 5);
    println!("3*5={}", ex1);
    let ex2 = multiplication(8, 4);
    println!("8*4={}", ex2);
}
