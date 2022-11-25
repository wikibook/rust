use std::time::Instant;
fn main() {
    // 구하려는 피보나치 수 목록
    let request_nums = [43, 42, 20, 39, 37, 35, 30];
    let start_time = Instant::now();
    // 순차적으로 계산
    for num in request_nums {
        let answer = fib(num);
        println!("[결과] fib({} 번째 수) = {}", num, answer);
    }
    show_time(start_time);
}
// 피보나치 수를 구하는 재귀함수
fn fib(n: i64) -> i64 {
    if n == 1 { return 0; }
    if n == 2 { return 1; }
    return fib(n - 2) + fib(n - 1);
}
fn show_time(start_time: Instant) {
    let elapsed = start_time.elapsed();
    println!("실행 시간 : {:?}", elapsed);
}
