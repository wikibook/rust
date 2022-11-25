use std::time::{SystemTime, UNIX_EPOCH};

// 전역 변수로 이용할 난수 Seed 지정 --- (*1)
static mut SEED: u32 = 0;

// start부터 end 사이의 난수를 생성하는 unsafe 함수 --- (*2)
unsafe fn rand_global(start: u32, end: u32) -> u32 {
    // 필요하다면 Seed 값을 초기화 --- (*3)
    if SEED == 0 {
        // 현재 시각을 이용해 난수를 초기화
        let epoc = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap();
        SEED = epoc.as_millis() as u32;
    }
    // Seed를 이용해 난수 생성 --- (*4)
    SEED ^= SEED << 13;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;
    return SEED % (end - start + 1) + start;
}

fn main() {
    // 난수 100개 생성
    for _ in 0..100 {
        // 안전하지 않다는 것을 명시 --- (*5)
        unsafe {
            // 난수를 생성해 출력
            let v = rand_global(1, 6);
            println!("{}", v);
        }
    }
}