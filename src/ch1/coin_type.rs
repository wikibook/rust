fn main() {
    // 거스름돈 --- (*1)
    let price: i64 = 3950;
    // 각 동전이 몇 개 있는지 정의 --- (*2)
    let count500: i64 = 10;
    let count100: i64 = 3;
    let count50: i64  = 10;
    // 반복문을 통해 거스름돈 조합 계산
    for i500 in 0..(count500+1) {
        for i100 in 0..(count100+1) {
            for i50 in 0..(count50+1) {
                // 동전을 더한 금액을 계산 --- (*3)
                let total: i64 = 
                    i50 * 50 + i100 * 100 + i500 * 500;
                // 동전을 더한 금액과 거스름돈이 일치하면 출력
                if price == total {
                    println!("500원x{}+100x{}+50원x{}={}",
                        i500, i100, i50, total);
                }
            }
        }
    }
}
