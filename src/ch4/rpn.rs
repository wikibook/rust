use std::io;
fn main() {
    // 계산용 스택 --- (*1)
    let mut stack: Vec<f64> = vec![];
    // 표준 입력으로부터 수식 얻기 --- (*2)
    println!("RPN:");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("入力エラー");
    // 수식을 빈칸으로 분할해 배열로 만든 후 반복문을 통해 계산 --- (*3)
    let tokens = s.split_whitespace();
    for tok in tokens {
        let t = tok.trim();
        // 숫자 값이라면 스택에 PUSH --- (*4)
        match t.parse::<f64>() {
            Ok(v) => { stack.push(v); continue; },
            Err(_) => 0.0,
        };
        // 연산자라면 2번 POP을 하고 2개의 값을 계산한 결과를 PUSH --- (*5)
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        match t {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "*" => stack.push(a * b),
            "/" => stack.push(a / b),
            _ => panic!("계산이 불가능한 연산자 : {}",t),
        }
    }
    // 결과 표시 --- (*6)
    println!("{}", stack.pop().unwrap());
}
