fn main() {
    // 문자열에 숫자 값을 대입
    let s = "365";
    // i32 타입 숫자 값으로 변환
    let i: i32  = match s.parse() {
        Ok(v) => v,   // 성공했을 때
        Err(_) => 0,  // 실패했을 때
    };
    println!("{}", i);
}

