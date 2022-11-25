fn main() {
    // 문자열 생성
    let mut s = String::from("벼는 익을수록 고개를 숙인다");
    // 연속으로 가변 참조를 사용하려면 스코프를 나눈다
    {
        let ref1 = &mut s;
        println!("ref1={}", ref1);
    } // 여기서 ref1은 파기된다
    {
        let ref2 = &mut s;
        println!("ref2={}", ref2);
    }
}

