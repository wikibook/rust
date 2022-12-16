// 키와 몸무게 데이터를 가지는 구조체 --- (*1)
struct Body {
    weight: f64,
    height: f64,
}

fn main() {
    // 구조체 초기화 --- (*2)
    let hong = Body {
        weight: 80.0,
        height: 165.0
    };
    let lim = Body {
        weight: 65.0,
        height: 170.0,
    };
    // 함수 호출 --- (*3)
    println!("홍길동 = {:.1}", calc_bmi(&hong));
    println!("임꺽정 = {:.1}", calc_bmi(&lim));
}

// BMI를 계산하는 함수 --- (*4)
fn calc_bmi(body: &Body) -> f64 {
    let h = body.height / 100.0;
    body.weight / h.powf(2.0)
}
