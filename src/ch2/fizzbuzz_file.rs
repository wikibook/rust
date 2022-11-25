use std::fs::{self, File};
use std::io::{Write, BufWriter};

fn main() {
    // 저장할 파일 이름 지정 --- (*1)
    let filename = "fizzbuzz_file_result.txt";
    // 파일로 저장할 부분을 블록으로 지정(Scope 지정) --- (*2)
    {
        // 파일 생성 및 열기 --- (*3)
        let fp = File::create(filename).unwrap();
        let mut writer = BufWriter::new(fp);
        // FizzBuzz를 100까지 구하기
        for i in 1..=100 {
            let mut line = format!("{}\n", i);
            if (i % 3 == 0) && (i % 5 == 0) {
                line = String::from("FizzBuzz\n");
            } else if i % 3 == 0 {
                line = String::from("Fizz\n");
            } else if i % 5 == 0 {
                line = String::from("Buzz\n");
            }
            // 파일에 쓰기--- (*4)
            let bytes = line.as_bytes();
            writer.write(bytes).unwrap();
        }
    } // ← 여기서 파일은 자동으로 닫힌다 --- (*5)
    
    // 저장한 파일의 내용을 읽어들여 출력 --- (*6)
    let s = fs::read_to_string(filename).unwrap();
    println!("{}", s);
}