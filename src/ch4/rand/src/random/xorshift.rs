// XorShift로 의사 난수 생성
pub fn rand(seed: &mut u32) -> u32 {
    let mut y = *seed;
    y ^= y << 13;
    y ^= y >> 17;
    y ^= y << 5;
    *seed = y;
    y
}
