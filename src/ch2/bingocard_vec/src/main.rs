// 배열을 섞기 위한 rand 크레이트 이용 
use rand::seq::SliceRandom;

fn main() {
    // 1에서 75까지의 값을 벡터에 대입 --- (*1)
    let mut nums = vec![];
    for i in 1..=75 { nums.push(i); }
    
    // 섞기 --- (*2)
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    // 카드 표시 --- (*3
    for i in 0..25 {
        if i == 12 {
            print!("  *,");  // 와일드 카드
        } else {
            print!("{:3},", nums[i]);
        }
        if i % 5 == 4 { println!(""); }
    }
}