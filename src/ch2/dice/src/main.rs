// 주사위를 5번 굴리기
// rand 크레이트의 Rng를 이용 --- (*1)
use rand::Rng;

fn main() {
    // 난수 생성기 준비 --- (*2)
    let mut rng = rand::thread_rng();
    // 5번 반복 수행
    for _ in 0..5 {
        // 1~6 사이의 난수를 생성 --- (*3)
        let dice = rng.gen_range(1..=6);
        println!("{}", dice);
    }
}