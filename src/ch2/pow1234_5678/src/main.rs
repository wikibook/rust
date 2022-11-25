extern crate num_bigint;
// BigInt를 사용하기 위한 선언 --- (*1)
use num_bigint::BigInt;

fn main() {
    // BigInt 오브젝트를 만들어 값을 설정 --- (*2)
    let v = BigInt::from(1234);
    // 5678 제곱 계산 --- (*3)
    println!("{}", v.pow(5678));
}
