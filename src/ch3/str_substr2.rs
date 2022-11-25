fn main() {
    let pr = "지혜는 무기보다 가치가 있다.";

    // 앞의 2문자 부분 문자열 얻기 --- (*1)
    let sub3: String = pr.chars().take(2).collect();
    println!("앞 2문자: {}", sub3);

    // '무기'부분 문자열 얻기 --- (*2)
    let pr_chars: Vec<char> = pr.chars().collect(); // 변환
    //  4-5번째 문자열 얻기
    let sub_chars = &pr_chars[4..=5]; // 슬라이스 
    // 슬라이스를 문자열로 변환
    let sub4: String = sub_chars.into_iter().collect();
    println!("4-5번째 문자: {}", sub4);
}
