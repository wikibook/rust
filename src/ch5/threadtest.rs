use std::{thread, time};

// 3초간 1초에 한 번 메시지를 표시하는 함수 --- (*1)
fn sleep_print(word: &str) {
    for i in 1..=3 {
        println!("{}: i={}", word, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    // 스레드를 이용하지 않는 경우 --- (*2)
    println!("--- 스레드 없음 ---");
    sleep_print("스레드 없음");
    
    // 스레드를 이용하는 경우 --- (*3)
    println!("--- 스레드 이용 ---");
    // 스레드 1
    thread::spawn(|| {
        sleep_print("토마토")
    });
    // 스레드 2
    thread::spawn(|| {
        sleep_print("스위스")
    });
    // 스레드 3
    thread::spawn(|| {
        sleep_print("별똥별")
    });
    // 메인 스레드
    sleep_print("기러기");
}
