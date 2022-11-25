// 피보나치 수열을 반환하는 반복자 --- (*1)
struct FibIterator {
    a: usize,
    b: usize,
}
impl FibIterator {
    fn new() -> Self { FibIterator {a: 1, b: 1} }
}
// 반복자 구현 --- (*2)
impl Iterator for FibIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.a;
        self.a = self.b;
        self.b += tmp;
        return Some(self.a);
    }
}

fn main() {
    // for를 이용해 결과를 10개 출력 --- (*3)
    let fib_iter = FibIterator::new();
    for (i, n) in fib_iter.enumerate() {
        if i >= 10 { break; }
        print!("{},", n);
    }
    println!("");
    // take를 이용하는 경우 --- (*4)
    let fib_iter = FibIterator::new();
    fib_iter.take(10).for_each(|f| print!("{},", f));
    print!("\n")
}