// BMI 판정용 구조체 --- (*1)
struct BmiRange {
    min: f64,            // min이상
    max: f64,            // max미만
    label: &'static str, // 판정
}
fn main() {
    // 키와 몸무게 입력\ --- (*2)
    let height_cm = input("키(cm) : ");
    let weight = input("몸무게(kg) : ");
    // BMI 계산
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    // 비만도 판정표를 벡터 타입으로 생성 --- (*3)
    let bmi_list = vec![
        BmiRange {
            min: 0.0,
            max: 18.5,
            label: "저체중",
        },
        BmiRange {
            min: 18.5,
            max: 23.0,
            label: "정상",
        },
        BmiRange {
            min: 23.0,
            max: 25.0,
            label: "비만전단계",
        },
        BmiRange {
            min: 25.0,
            max: 30.0,
            label: "1단계 비만",
        },
        BmiRange {
            min: 30.0,
            max: 35.0,
            label: "2단계 비만",
        },
        BmiRange {
            min: 35.0,
            max: 99.0,
            label: "3단계 비만",
        },
    ];
    // 비만도 판단 --- (*4)
    let mut result = "계산 불가";
    for range in bmi_list {
        if range.min <= bmi && bmi < range.max {
            result = range.label;
            break;
        }
    }
    // 결과 표시
    println!("BMI = {:.1}, 비만도 = {}", bmi, result);
}
// 한 줄씩 읽어 f64 타입으로 반환 --- (*5)
fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("입력 에러");
    s.trim().parse().expect("숫자 변환 에러")
}
