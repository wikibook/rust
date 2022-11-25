use std::num::Wrapping;

// 선형 합동법으로 의사 난수 생성
pub fn rand(seed: &mut u32) -> u32 {
    let (a, c) = (134775813u32, 12345u32);
    *seed = (Wrapping(*seed) * Wrapping(a) + Wrapping(c)).0;
    *seed
}
