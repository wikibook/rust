// 파일 조작용 라이브러리 이용 선언 --- (*1)
use std::fs;
fn main() {
    // 파일 이름 지정 --- (*2)
    let afile = "./fizzbuzz_python.txt";
    let bfile = "./fizzbuzz_rust.txt";
    
    // 파일 내용을 문자열로 읽어들임 --- (*3)
    let astr = fs::read_to_string(afile).unwrap();
    let bstr = fs::read_to_string(bfile).unwrap();
    
    // 불필요한 공백 삭제
    let astr = astr.trim();
    let bstr = bstr.trim();

    // 비교 --- (*4)
    if astr == bstr {
        println!("ok");
    } else {
        println!("ng");
    }
}