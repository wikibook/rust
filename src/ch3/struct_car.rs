// CarSpec 구조체 정의 --- (*1)
struct CarSpec {
    model: i32, // 모델
    cc: i32, // 배기량
    color: i32, // 색상
}

fn main() {
    // CarSpec 객체 생성 --- (*2)
    let car1 = CarSpec {
        model: 3001,
        cc: 1500,
        color: 0xFF0000,
    };
    let car2 = CarSpec {
        model: 3002,
        cc: 1200,
        color: 0x0000FF,
    };
    // 객체의 각 필드를 출력 --- (*3)
    println!("car1: {}, {}cc, {:06x}", 
        car1.model, car1.cc, car1.color);
    println!("car2: {}, {}cc, {:06x}", 
        car2.model, car2.cc, car2.color);
}