fn main() {
    // 첫 번째 문자를 출력 --- (*1)
    let s2 = "abcdefg";
    println!("{}", &s2[0..1]); // a

    let s = "안녕하세요";

    // 첫 번째 1문자를 출력 --- (*2)
    let ch = &s[..3];
    println!("{}", ch); // 안
    
    // 세 번째 1문자를 출력 --- (*3)
    let ch = &s[6..9];
    println!("{}", ch); // 하
}