use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 난수 초기화 --- (*1)
    let mut seed = rand_init();
    // 100개의 난수를 만들기 위한 반복문
    for _ in 0..100 {
        // 난수 생성 --- (*2)
        let v = rand(&mut seed, 1, 6);
        println!("{}", v);
    }
}

// 난수를 초기화하는 함수 --- (*3)
fn rand_init() -> u32 {
    // 현재 시각을 이용해 난수를 초기화
    SystemTime::now()
      .duration_since(UNIX_EPOCH).unwrap()
      .as_millis() as u32
}

// start부터 end 사이의 난수를 생성하는 함수 --- (*4)
fn rand(seed: &mut u32, start: u32, end: u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    return *seed % (end - start + 1) + start;
}
