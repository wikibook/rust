fn main() {
    // 문자열 생성
    let mut s = String::from("벼는 익을수록 고개를 숙인다");
    // 참조 값 얻기
    let ref1 = &mut s;
    let ref2 = &mut s; // ← 에러 발생
    // 값 출력
    println!("ref1={}, ref2={}", ref1, ref2);
}

