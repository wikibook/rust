// 배열을 섞기 위한 rand 크레이트 이용 선언 --- (*1)
use rand::seq::SliceRandom;

fn main() {
    // 1에서 75까지의 숫자로 이루어진 배열을 생성 --- (*2)
    let mut nums = [0; 75];
    for i in 1..=75 { nums[i-1] = i; }

    // 섞기 --- (*3)
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);
    
    // 카드 표시 --- (*4)
    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 { // 와일드 카드
                print!("  *,");
            } else {
                print!("{:3},", nums[i]);
            }
        }
        println!("");
    }
}
