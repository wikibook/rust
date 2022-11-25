// 재귀적으로 호출할 함수 sum
fn sum(n:i32) -> i32 {
    if n <= 0 { return 0; } // 재귀 종료 조건 --- (*1)
    return sum(n-1) + n; // 재귀 호출 --- (*2)
}

fn main() {
    println!("{}", sum(10));
}