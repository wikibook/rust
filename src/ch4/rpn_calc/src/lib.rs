// 설명 작성 --- (*1)
//! # RPN Calc
//! Reverse Polish notation (RPN) Calc.
//! # Example
//! ```
//! let src = String::from("1 2 + 3 * ");
//! let a = rpn_calc::eval(src).unwrap();
//! println!("{}", a); // →9
//! ```

pub fn eval(src: String) -> Result<f64, String> {
    // 인수를 공백으로 구분
    let tokens = src.split_whitespace();
    let mut stack:Vec<f64> = vec![];
    // 반복문으로 요소를 계산 --- (*2)
    for tok in tokens {
        let t = tok.trim();
        if t == "" { continue; }
        // 숫자 값이라면 스택에 PUSH
        match t.parse::<f64>() {
            Ok(v) => { stack.push(v); continue; },
            Err(_) => 0.0,
        };
        // 연산자라면 2번 POP 한 뒤 계산 결과를 PUSH
        let b = stack.pop().unwrap_or(0.0);
        let a = stack.pop().unwrap_or(0.0);
        match t {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "*" => stack.push(a * b),
            "/" => stack.push(a / b),
            "%" => stack.push(a % b),
            _ => return Err(format!("invalid operator: {}", t)),
        }
    }
    // 결과 반환 --- (*3)
    if stack.len() == 0 { return Err(format!("no result")); }
    if stack.len() > 1 { 
        return Err(format!("too many value in stack")); 
    } 
    Ok(stack.pop().unwrap_or(0.0))
}

// 테스트 --- (*4)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(eval("1 3 +".to_string()), Ok(4.0));
        assert_eq!(eval("2 3 *".to_string()), Ok(6.0));
        assert_eq!(eval("6 3 /".to_string()), Ok(2.0));
        assert_eq!(eval("6 3 - 1 -".to_string()), Ok(2.0));
    }
}
