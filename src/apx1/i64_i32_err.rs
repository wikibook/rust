fn main() {
    // i32 타입 변수를 정의
    let n: i32 = 100;
    // i64 타입에 i32 타입 값을 대입
    let m: i64 = n; // ← 에러
    println!("{},{}", n, m);
}

