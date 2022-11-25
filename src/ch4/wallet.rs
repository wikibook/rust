// 화폐 종류를 나타내기 위한 열거형 --- (*1)
enum Currency {
    Currency100(isize),
    Currency500(isize),
    Currency1000(isize),
    Currency5000(isize),
    Currency10000(isize),
    Currency50000(isize),
}
impl Currency {
    // 화폐 종류로 실제 금액 계산 --- (*2)
    fn calc_price(&self) -> isize {
        match *self {
            Currency::Currency100(v) => v * 100,
            Currency::Currency500(v) => v * 500,
            Currency::Currency1000(v) => v * 1000,
            Currency::Currency5000(v) => v * 5000,
            Currency::Currency10000(v) => v * 10000,
            Currency::Currency50000(v) => v * 50000,
        }
    }
}

fn main() {
    // 지갑 안에 있는 동전 종류와 갯수 지정 --- (*3)
    let wallet: Vec<Currency> = vec![
        Currency::Currency100(3),
        Currency::Currency500(2),
        Currency::Currency1000(6),
        Currency::Currency5000(1),
        Currency::Currency10000(8),
        Currency::Currency50000(3),
    ];
    // 전체 금액을 계산해서 출력 --- (*4)
    let total = wallet.iter()
        .fold(0, |sum, v| sum + v.calc_price());
    println!("지갑 안의 금액은 {} 원입니다", total);
}

