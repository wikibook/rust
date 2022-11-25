fn main() {
    // u32 타입 벡터 생성
    let a_vec: Vec<u32> = vec![100, 200, 300];
    for i in a_vec {
        println!("{}", i);
    }
    // &str 타입 벡터 생성
    let s_vec: Vec<&str> = vec!["개", "고양이", "닭"];
    for i in s_vec {
        println!("{}", i);
    }
}