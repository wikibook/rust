fn main() {
    let a = 'a';  // 문자'a'를 지정
    let b = b'a'; // ASCII코드 97u8 을 지정
    let c = '\x61'; // 16진수로 문자 'a'를 지정
    println!("{},{:2x},{}", a, b, c);

    let d = '곰'; // 문자 '곰'을 지정
    let e = '곰' as u32; // 문자 '곰'의 문자 코드 'acf0'를 지정
    let f = '\u{acf0}'; // 16진수로 문자 '곰'을 지정
    println!("{},{:4x},{}", d, e, f);
}