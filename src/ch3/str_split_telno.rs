fn main() {
    // 전화번호 지정
    let telno = "955-3658";
    
    // 슬라이스로 분할 --- (*1)
    println!("-- 슬라이스 --");
    println!("국번: {}", &telno[..3]);
    println!("사번: {}", &telno[4..]);
    
    // split_at으로 분할 --- (*2)
    println!("--- split_at ---");
    let (telno1, telno2) = telno.split_at(3);
    let (telno2, telno3) = telno2.split_at(1);
    println!("국번: {}", telno1);
    println!("구분: {}", telno2);
    println!("사번: {}", telno3);
    
    // split_off로 분할 --- (*3)
    println!("-- split_off --");
    let mut telno1 = String::from(telno);
    let mut telno2 = telno1.split_off(5);
    println!("국번ㄲㄱ: {}", telno1);
    let telno3 = telno2.split_off(1);
    println!("국번: {}", telno1);
    println!("구분: {}", telno2);
    println!("사번: {}", telno3);

    // split으로 분할 --- (*4)
    println!("-- split --");
    let telno_a: Vec<&str> = telno.split('-').collect();
    println!("국번: {}", telno_a[0]);
    println!("사번: {}", telno_a[1]);
}