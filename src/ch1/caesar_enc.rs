// 암호화 함수
fn encrypt(text: &str, shift: i16) -> String {
    // 'A'와'Z'의 문자코드를 i16 타입으로 취득 --- (*1)
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    // 결과를 대입할 변수를 선언
    let mut result = String::new();
    // 한 글자씩 치환 처리 --- (*2)
    for ch in text.chars() {
        // 문자코드를 변환
        let mut code = ch as i16;
        // A와Z 사이에 있는 값인가？ --- (*3)
        if code_a <= code && code <= code_z {
            // shift만큼 뒤의 문자로 치환 --- (*4)
            code = (code - code_a + shift + 26) % 26 + code_a;
        }
        // 문자코드를 다시 문자로 변환 --- (*5)
        result.push((code as u8) as char);
    }
    return result;
}

fn main() {
    // 함수 호출
    let enc = encrypt("I LOVE RUST.", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);
}
