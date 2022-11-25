fn main() {
    hex_dump("성공하는 사람은 송곳처럼 어느 한 점을 향하여 일한다.");
}

fn hex_dump(s: &str) {
    // １바이트씩 표시 --- (*1)
    for (i, c) in s.bytes().enumerate() {
        // 주소를 표시
        if i % 16 == 0 {
            print!("{:08x}|", i);
        }
        // 4자리씩 끊어 문자를 표시
        if i % 4 == 3 {
            print!("{:02x}|", c);
        } else {
            print!("{:02x} ", c);
        }
        // 16바이트마다 줄바꿈
        if i % 16 == 15 {
            println!("");
        }
    }
    println!("");
}