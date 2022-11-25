peg::parser!( grammar calc() for str {
    
    // 기본이 되는 규칙 --- (*1)
    pub rule eval() -> i64 
        = expr()
    
    // 덧셈과 뺄셈용 규칙 추가 --- (*2)
    rule expr() -> i64
        = l:term() "+" r:expr()   { l + r }
        / l:term() "-" r:expr()   { l - r }
        / term()

    // 곱셈과 나눗셈용 규칙 추가 --- (*3)
    rule term() -> i64
        = l:value() "*" r:term() { l * r }
        / l:value() "/" r:term() { l / r }
        / v:value()

    // 값을 읽는 규칙 추가 --- (*4)
    rule value() -> i64
        = number()                  // 숫자 값
        / "(" v:expr() ")" { v }   // (계산식)
        
    // 숫자 값을 읽는 규칙 추가 --- (*5)
    rule number() -> i64
        = n:$(['0'..='9']+) 
        { n.parse().unwrap() }
});

fn main() {
    // 계산
    println!("{}", calc::eval("1+2*3").unwrap());
    println!("{}", calc::eval("(1+2)*3").unwrap());
    println!("{}", calc::eval("100/2-1").unwrap());    
}