fn main() {
    let pr = "구슬이 서 말이라도 꿰어야 보배";
    // 1자씩 표시 --- (*1)
    for c in pr.chars() {
        print!("[{}]", c);
    }
    // 글자 수를 세기 --- (*2)
    println!("\n글자 수 = {}자", pr.chars().count());

    // Vec<char>로 변환해 처리 --- (*3)
    let pr_chars: Vec<char> = pr.chars().collect();
    println!("Vec<char> : {:?}", pr_chars);
    for c in pr_chars.iter() {
        print!("({})", c);
    }
    println!("\n글자 수 = {}자", pr_chars.len());
}