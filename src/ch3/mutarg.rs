// 인수의 값에 2를 곱해 반환하는 함수 --- (*1)
fn x2(arg: &mut i32) {
    *arg = *arg * 2;
}

fn main() {
    let mut v = 16;
    x2(&mut v); // 인수에 2가 곱해진다 --- (*2)
    println!("{}", v);
}