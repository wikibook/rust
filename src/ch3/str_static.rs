// &'static str을 이용하는 함수 --- (*1)
fn echo(s: &'static str) {
    println!("{}", s);
}
fn main() {
    // 문자열 리터럴(&'static str)을 지정 --- (*2)
    echo("웅변은 은이요");
    echo("침묵은 금이다");

    // 아래 주석 부분은 에러가 발생한다 --- (*3)
    // let s = String::from("테스트");
    // echo(&s);
}