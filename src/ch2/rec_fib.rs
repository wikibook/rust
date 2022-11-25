// 피보나치 수를 구하는 함수
fn fib(n:i64) -> i64 {
    if n == 1 { return 0; } // 재귀 호출 종료 조건 --- (*1)
    if n == 2 { return 1; } //
    return fib(n-2) + fib(n-1); // 재귀 호출 --- (*2)
}

fn main() {
    for i in 2..=20 {
        println!("{}", fib(i));
    }
}