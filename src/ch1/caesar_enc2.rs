// 암호화 함수
fn encrypt(text: &str, shift: i16) -> String {
    let a = 'A' as i16;
    let is_az = |c| 'A' <= c && c <= 'Z';
    let conv = |c| (((c-a+shift+26)%26+a) as u8)as char;
    let enc1 = |c| if is_az(c) { conv(c as i16) } else { c };
    text.chars().map(|c| enc1(c)).collect()
}

fn main() {
    let enc = encrypt("I LOVE RUST.", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);
}