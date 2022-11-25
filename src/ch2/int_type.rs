use std::any::type_name;

// 타입 확인을 위한 함수 정의
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let a = 100u8;   // u8 타입 100
    let b = 100i128; // i128 타입 100
    let c = 10_000; // 10000 과 동일한 의미
    println!("각 타입 확인 : a={}({}), b={}({}), c={}({})", a, type_of(a), b, type_of(b), c, type_of(c));
}

