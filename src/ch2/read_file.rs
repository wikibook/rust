use std::env; // 명령줄 인수 취득용
use std::fs; // 파일 읽기용

fn main() {
    // 인수를 벡터로 취득 --- (*1)
    let args: Vec<String> = env::args().collect();
    // 인수를 지정했는지 확인 --- (*2)
    if args.len() < 2 {
        println!("읽어올 파일을 지정해주세요.");
        return;
    }
    // 두 번째 요소 --- (*3)
    let filename = &args[1];
    // 파일을 읽어와 출력 --- (*4)
    let text = fs::read_to_string(filename).unwrap();
    println!("{}", text);
}