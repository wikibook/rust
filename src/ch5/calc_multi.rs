use std::sync::mpsc;
use std::thread;
use std::time::{Instant, Duration};

fn main() {
    // 구하려는 피보나치 수 목록
    let request_nums = [43, 42, 20, 39, 37, 35, 30];
    let start_time = Instant::now();
    // 스레드간 통신 채널 생성 --- (*1)
    let (tx, rx) = mpsc::channel::<(i64,i64)>();
    // 연속해서 스레드를 생성해 계산 수행 --- (*2)
    for num in request_nums {
        let sender = tx.clone();
        thread::spawn(move || {
            let answer = fib(num);
            sender.send((num, answer)).unwrap();
        });
    }
    // 생성한 스레드의 수 구하기 --- (*3)
    let mut job = request_nums.len();
    
    // 계산 결과 얻기 --- (*4)
    loop {
        if let Ok((arg, answer)) = rx.recv() {
            job -= 1;
            println!("[결과] fib({} 번째 수) = {} (남은 계산={})", 
                arg, answer, job);
            if job <= 0 {
                show_time(start_time);
                break;
            }
        }
        thread::sleep(Duration::from_millis(300));
    }
}
// 피보나치 수를 구하는 재귀함수
fn fib(n: i64) -> i64 {
    if n == 1 { return 0; }
    if n == 2 { return 1; }
    return fib(n-2) + fib(n-1);
}
fn show_time(start_time: Instant) {
    let elapsed = start_time.elapsed();
    println!("실행 시간 : {:?}", elapsed);
}
