fn main() {
    let g1 = 30;
    let g2 = g1; // 값이 자동으로 복사됨 --- (*1)
    println!("{}", g1); // ok
    println!("{}", g2); // ok
}