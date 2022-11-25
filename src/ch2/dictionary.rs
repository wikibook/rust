use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    // 사전 파일 지정 --- (*1)
    let dicfile = "dict.txt";
    
    // 명령줄 인수를 벡터에 할당 --- (*2)
    let args: Vec<String> = std::env::args().collect();
    
    // 인수 확인
    if args.len() < 2 {
        println!("[USAGE] ./dictionary word");
        return;
    }
    // 인수로 전달된 단어 --- (*3)
    let word = &args[1];
    
    // 사전 파일 열기 --- (*4)
    let fp = File::open(dicfile).unwrap();
    // BufReader로 읽어들임 --- (*5)
    let reader = BufReader::new(fp);
    for line in reader.lines() {
        // 한 줄씩 처리 --- (*6)
        let line = line.unwrap();
        // 지정한 단어가 포함되는 줄인지 확인 --- (*7)
        if line.find(word) == None { 
            continue; 
        }
        println!("{}", line);
    }
}