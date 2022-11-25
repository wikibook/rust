// 숫자 값을 출력하는 매크로 --- (*1)
macro_rules! echo_num {
    ($num:expr) => { println!("{}", $num); }
}

// 매크로 이용 --- (*2)
fn main() {
    echo_num!(10);
    echo_num![20];
    echo_num!{30}
}
