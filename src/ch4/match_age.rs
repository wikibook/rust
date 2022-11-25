fn main() {
    let age = 8;
    let age_str = match age {
        0 => "유아",
        1..=10 => "어린이",
        11..=18 => "청소년",
        _ => "어른",
    };
    println!("{}살은 {} 요금입니다.", age, age_str);
}
