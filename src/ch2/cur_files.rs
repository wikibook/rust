use std::fs;
fn main() {
    // 파일 목록 취득 --- (*1)
    let files = fs::read_dir(".").expect("올바르지 않은 경로입니다");
    for ent in files {
        // 목록을 하나씩 처리 --- (*2)
        let entry = ent.unwrap();
        // PathBuf 오브젝트 얻기 --- (*3)
        let path = entry.path();
        // 파일 이름 출력 --- (*4)
        let fname = path.to_str().unwrap_or("올바르지 않은 파일 이름입니다");
        println!("{}", fname);
    }
}