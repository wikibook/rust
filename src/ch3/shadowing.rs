fn main() {
    // 섀도잉을 이용하지 않음 --- (*1)
    {
        let mut v = 300; // v를 가변 변수로 선언
        v = v + 5;
        println!("{}", v);
    }
    // 섀도잉을 이용 --- (*2)
    {
        let v = 300; // v는 불변 변수
        let v = v + 5;
        println!("{}", v);
    }
}