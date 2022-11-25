// 인수 값을 2배로 만드는 제네릭
fn x2 <T: std::ops::Add<Output=T> + Copy> (n: T) -> T {
    n + n
}
fn main() {
    println!("{}", x2(3));
    println!("{}", x2(3.0f64));
    println!("{}", x2::<u64>(3));
}