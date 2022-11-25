// BMI와 비만도를 출력하는 함수 --- (*1)
fn print_bmi(height: f32, weight: Option<f32>) {
    // weight에 값이 있다면 BMI를 구해 Option 타입으로 반환 --- (*2)
    let bmi:Option<f32> = match weight {
        Some(w) => Some(w / (height / 100.0).powf(2.0)),
        None => None,
    };
    // BMI 값에 따라 비만도 판단 --- (*3)
    let msg = match bmi {
        Some(n) if n < 18.5 => "저체중",
        Some(n) if n < 23.0 => "정상",
        Some(n) if n < 25.0 => "비만전단계",
        Some(n) if n < 30.0 => "1단계 비만",
        Some(n) if n < 35.0 => "2단계 비만",
        Some(_) => "3단계 비만",
        None => "계산 불가",
    };
    // 판단 결과 출력 --- (*4)
    println!("BMI = {:.1}, 결과 = {}", bmi.unwrap(), msg);
}
// 몇 가지 입력 만들기 --- (*5)
fn main() {
    let height = 162.3;
    print_bmi(height, Some(48.0));
    print_bmi(height, Some(72.3));
    print_bmi(height, None);
}