// 2개 이상의 인수를 표시할 수 있는 매크로 정의 --- (*1)
#[macro_export]
macro_rules! echo_nums {
    ( $( $num:expr ),* ) => {
        $(
            print!("{}, ", $num);
        )*
        println!("");
    }
}

// 매크로 이용 --- (*2)
fn main() {
    echo_nums![10, 20, 30, 40, 50];
    echo_nums!(60, 70);
    echo_nums!{80, 90, 100}
}