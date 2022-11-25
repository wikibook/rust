// 문법 규칙 시작 --- (*1)
peg::parser!( grammar calc() for str {
    // 기본이 되는 규칙 추가 --- (*2)
    pub rule eval() -> i64      // 규칙 이름
        = term()                // 구문 정의

    // 덧셈을 수행하는 규칙 추가 --- (*3)
    rule term() -> i64          // 규칙 이름
        = v1:num() "+" v2:num() // 구문 정의
        { v1 + v2 }             // 동작

    // 숫자 값을 읽는 규칙 추가 --- (*4)
    rule num() -> i64               // 규칙 이름
        = value:$(['0'..='9']+)     // 구문 정의
        { value.parse().unwrap() }  // 동작
});

fn main() {
    // 덧셈 계산식 실행 --- (*5)
    println!("2+5={}", calc::eval("2+5").unwrap());
    println!("8+2={}", calc::eval("8+2").unwrap());
    println!("200+50={}", calc::eval("200+50").unwrap());
}
