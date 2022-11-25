fn main() {
    // 문자열 리터럴은 &str 타입 --- (*1)
    let ss: &str = "베풀면 반드시 돌아온다";
    // &str을 String으로 변환 --- (*2)
    let so1: String = String::from(ss);
    let so2: String = ss.to_string();
    // String을 &str로 변환 --- (*3)
    let ss2: &str = &so1;
    let ss3: &str = so1.as_str();
    // 출력
    println!("{}\n{}\n{}\n{}", so1, so2, ss2, ss3);
    // 참조 타입 포인터 주소를 표시 --- (*4)
    println!("{:p}\n{:p}", ss2, ss3);
}