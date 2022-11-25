fn main() {
    let g1 = String::from("온화한 마음은 몸에 좋다");
    let g2 = g1.clone(); // 복제하면 소유권은 이동하지 않는다
    println!("{}", g1); // ok
    println!("{}", g2); // ok
}