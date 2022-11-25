// 비만도 진단 도구
fn main() {
    // 키와 몸무게 입력 --- (*1)
    let height_cm = input("키(cm) : ");
    let weight = input("몸무게(kg) : ");
    // BMI 계산 --- (*2)
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI={:.1}", bmi);
    // 비만도 진단 --- (*3)
    if bmi < 18.5 { println!("저체중"); }
    else if bmi < 23.0 { println!("정상"); }
    else if bmi < 25.0 { println!("비만전단계"); }
    else if bmi < 30.0 { println!("1단계 비만"); }
    else if bmi < 35.0 { println!("2단계 비만"); }
    else { println!("3단계 비만"); }
}

// 표준 입력에서 1줄씩 읽어 f64 타입으로 반환하는 함수 --- (*4)
fn input(prompt: &str) -> f64 {
    // 메시지 출력
    println!("{}", prompt); 
    // 입력 값을 가져옴 --- (*5)
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("입력 에러");
    // 공백을 제거하고 숫자 값으로 변환 --- (*6)
    return s.trim().parse().expect("숫자가 아닙니다.");
}