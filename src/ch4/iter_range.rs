fn main() {
    // 1부터 7 사이의 홀수만을 출력
    for i in 1..=7 {
        if i % 2 == 1 {
            println!("{}", i);
        }
    }
}