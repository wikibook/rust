// 피보나치 수열 구하기
fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut tmp: i32;
    println!("{}", a);
    println!("{}", b);
    for _ in 0..30 {
        println!("{}", a+b);
        tmp = a;
        a = b;
        b = tmp + b;
    }
}

