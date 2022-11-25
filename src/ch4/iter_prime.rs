// 소수를 반환하는 8비트 반복자
struct PrimeIterator {
    n: u8,
}
// 메서드 정의 --- (*1)
impl PrimeIterator {
    fn new() -> Self { PrimeIterator {n: 1} }
    // self.n이 소수인지 확인
    fn is_prime(&self) -> bool {
        for i in 2..self.n {
            if self.n % i == 0 { return false; }
        }
        return true;
    }
}
// 반복자 구현 --- (*2)
impl Iterator for PrimeIterator {
    type Item = u8; // --- (*3)
    // 다음 소수 값을 반환 --- (*4)
    fn next(&mut self) -> Option<Self::Item> {
        // 소수를 찾아 반환
        loop {
            self.n += 1;
            // 8비트를 넘는 소수는 찾지 않음 --- (*5)
            if std::u8::MAX == self.n {
                return None
            }
            // self.n이 소수인지 확인 --- (*6)
            if self.is_prime() { return Some(self.n); }
        }
    }
}

fn main() {
    // 반복자 생성 --- (*7)
    let prime_iter = PrimeIterator::new();
    // for문으로 반복
    for n in prime_iter {
        print!("{},", n);
    }
}
