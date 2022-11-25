// 비만도 판정 매크로 --- (*1)
macro_rules! bmi_select {
    // 패턴 지정
    ( $bmi:expr, $( $label:expr => $range:expr );+) => {{
        // 매크로 기본 반환 값
        let mut result = "계산 불가";
        // 반복 --- (*2)
        $(
            if $range.start <= $bmi && $bmi < $range.end {
                result = $label;
            }
        )+
        result
    }};
}

fn main() {
    // 키와 몸무게 지정
    let h: f32 = 158.0;
    let w: f32 = 63.0;
    let bmi = w / (h / 100.0).powf(2.0);
    // 비만도 판정 매크로 이용 --- (*3)
    let label = bmi_select![
        bmi,
        "저체중" =>  0.0..18.5;
        "정상"   => 18.5..23.0;
        "비만전단계"  => 23.0..25.0;
        "1단계 비만"  => 25.0..30.0;
        "2단계 비만"  => 30.0..35.0;
        "3단계 비만"  => 35.0..99.9];
    println!("결과 : {}", label);
}
