fn main() {
    // 변수 선언
    let pc_price = 980000.0;
    let a_ship_fee = 12000.0;
    let a_rate = 0.8;
    let b_ship_fee = 0.0;
    let b_rate = 0.9;
    // 구입 비용 계산
    println!("A 쇼핑몰={}원", pc_price * a_rate + a_ship_fee);
    println!("B 쇼핑몰={}원", pc_price * b_rate + b_ship_fee);
}
