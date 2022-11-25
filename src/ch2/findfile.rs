use std::{env, path};

fn main() {
    // 명령줄 인수 확인 --- (*1)
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("findfile (path) (keyword)");
        return;
    }
    // 명령줄 인수 값 얻기 --- (*2)
    let target_dir = &args[1];
    let keyword = &args[2];
    // PathBuf로 변환 --- (*3)
    let target = path::PathBuf::from(target_dir);    
    findfile(&target, keyword);
}

// 재귀적으로 파일을 검색하는 함수 --- (*4)
fn findfile(target: &path::PathBuf, keyword: &str) {
    // 파일 목록을 취득 --- (*5)
    let files = target.read_dir().expect("존재하지 않는 경로");
    for dir_entry in files {
        // PathBuf로 경로 취득 --- (*6)
        let path = dir_entry.unwrap().path();
        // 디렉토리라면 자신을 다시 호출해 파일을 검색 --- (*7)
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }
        // 파일 이름을 문자열로 변환 --- (*8)
        let fname = path.file_name().unwrap()
            .to_string_lossy();
        // 검색어(파일 이름)를 포함하는지 확인 --- (*9)
        if None == fname.find(keyword) { continue; }
        // 검색어를 포함하는 경로를 표시
        println!("{}", path.to_string_lossy());
    }
}