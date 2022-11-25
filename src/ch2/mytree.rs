use std::{env, path};
fn main() {
    // 명령줄 인수 취득 --- (*1)
    let args: Vec<String> = env::args().collect();
    // 경로가 없는 경우 현재 디렉토리 지정
    let mut target_dir = "."; 
    if args.len() >= 2 { // 경로를 지정하는 경우
        target_dir = &args[1];
    }
    // PathBuf로 변환
    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    tree(&target, 0);
}

// 재귀적으로 파일 목록 표시 --- (*2)
fn tree(target: &path::PathBuf, level: isize) {
    // 파일 목록 취득 --- (*3)
    let files = target.read_dir().expect("존재하지 않는 경로입니다");
    // 반복해서 표시
    for ent in files {
        // PathBuf를 취득 --- (*4)
        let path = ent.unwrap().path();
        // level 만큼 들여쓰기 --- (*5)
        for _ in 1..=level {
            print!("|   ");
        }
        // 파일 이름 취득 --- (*6)
        let fname = path.file_name().unwrap()
            .to_string_lossy();
        // 디렉토리라면 재귀적으로 표시 --- (*7)
        if path.is_dir() {
            println!("|-- <{}>", fname);
            tree(&path, level+1);
            continue;
        }
        // 파일 이름 표시 --- (*8)
        println!("|-- {}", fname);
    }
}