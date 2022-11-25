fn main() {
    let g1 = String::from("온화한 마음은 몸에 좋다."); // --- (*1)
    let g2 = g1; // 소유권을 g2로 이동 --- (*2)
    println!("{}", g2); // --- (*3)
}