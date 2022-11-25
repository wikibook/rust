// random 모듈 선언 --- (*1)
mod random {
    // linear 모듈 선언 --- (*2)
    pub mod linear {
        use std::num::Wrapping;
        // 선형 합동법으로 난수 생성 --- (*3)
        pub fn rand(seed: &mut u32) -> u32 {
            let (a, c) = (134775813u32, 12345u32);
            *seed = (Wrapping(*seed) * Wrapping(a) +
                     Wrapping(c)).0;
            *seed
        }
    }
    // xorshift 모듈 선언 --- (*4)
    pub mod xorshift {
        // XorShift 로 난수 생성 --- (*5)
        pub fn rand(seed: &mut u32) -> u32 {
            let mut y = *seed;
            y ^= y << 13;
            y ^= y >> 17;
            y ^= y << 5;
            *seed = y;
            y
        }
    }
}

// 모듈 이용 선언 --- (*6)
use random::{linear, xorshift};
fn main() {
    // 각 알고리즘으로 10 개의 난수를 생성 --- (*7)
    let mut seed1 = 12345u32;
    let mut seed2 = 12345u32;
    for i in 0..10 {
        let r1 = linear::rand(&mut seed1) % 6 + 1;
        let r2 = xorshift::rand(&mut seed2) % 6 + 1;
        println!("L : {:2} 번째 = {}, {}", i+1, r1, r2);
    }
}