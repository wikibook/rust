fn main() {
    // 값을 2배로 해주는 클로저 정의
    let x2 = |n| n*2;
    // x2 이용
    println!("{}", x2(2));
    println!("{}", x2(8));
}
