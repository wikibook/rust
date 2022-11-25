use std::{env, fs};

fn main() {
    // 명령줄 인수 취득 --- (*1)
    let args = env::args();
    let mut total:f64 = 0.0;
    // 모든 인수를 처리 --- (*2)
    for (i, fname) in args.enumerate() {
        if i == 0 { continue; }
        // 텍스트 파일을 읽어들임 --- (*3)
        let text = fs::read_to_string(fname).unwrap();
        // 한 줄씩 분리 --- (*4)
        let lines = text.split('\n');
        // 반복해서 계산 --- (*5)
        for line in lines {
            // 숫자 값으로 변경 --- (*6)
            let n:f64 = match line.parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };
            total += n;
        }
    }
    // 결과 표시 --- (*7)
    println!("{}", total);
}