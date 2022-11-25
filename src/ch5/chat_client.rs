use std::io::{stdin, Write, BufRead, BufReader};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // 접속할 서버의 주소와 포트 지정 --- (*1)
    let server_addr = "127.0.0.1:8888";
    // 서버에 접속 --- (*2)
    let mut socket = TcpStream::connect(server_addr)
        .expect("서버에 접속할 수 없습니다.");
    socket.set_nonblocking(true).expect("알 수 없는 에러");
    println!("{}에 접속했습니다.", server_addr); 
    // 수신용 스레드 시작 --- (*3)
    start_thread(socket.try_clone().unwrap());

    // 표준 입력으로 사용자 이름 설정 --- (*4)
    let user = input("이름을 입력하세요.");
    println!("{}님 메시지를 입력해주세요", user);
    loop {
        // 표준 입력으로 입력받은 메시지를 서버로 전달 --- (*5)
        let msg = input("");
        let msg = format!("{}> {}\n", user, msg);
        let buf = msg.as_bytes();
        socket.write_all(buf).unwrap();
    }
}
// 스레드를 시작해 서버로부터 메시지를 수신 --- (*6)
fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        // 서버로부터 메시지를 수신
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n > 0 { // 수신한 내용을 화면에 표시
                println!("[받은 메시지] {}", buf.trim());
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}
// 표준 입력으로부터 문자열 얻기
fn input(msg: &str) -> String {
    if msg != "" { println!("{}", msg); }
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("입력 에러");
    String::from(buf.trim())
}
