mod cipher_str;
use std::env;

fn main() {
    // 명령줄 인수 확인
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        show_usage(); return;
    }
    // 명령줄 인수에서 값 얻기
    let method = String::from(args[1].trim());
    let password = String::from(args[2].trim());
    let data = String::from(args[3].trim());
    // 암호화・복호화
    let result = match &method[..] {
        "enc" => cipher_str::encrypt(&password, &data),
        "dec" => cipher_str::decrypt(&password, &data),
        _ => { show_usage(); return; },
    };
    println!("{}", result);
}

fn show_usage() {
    println!("[USAGE] cipher_cmd (enc|dec) password data");
}
