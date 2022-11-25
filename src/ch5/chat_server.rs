use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 서버 주소와 포트 번호 지정 --- (*1)
    let server_addr = "127.0.0.1:8888";
    // 스레드간 통신 준비 --- (*2)
    let (tx, rx) = mpsc::channel::<String>();
    // 클라이언트 목록 저장용 변수 선언 --- (*3)
    let mut clients: Vec<TcpStream> = Vec::new();

    // 서버 시작 --- (*4)
    let server = TcpListener::bind(server_addr).expect("서버 실행 실패");
    server.set_nonblocking(true).expect("알 수 없는 에러");
    println!("{}에서 서버가 실행 중입니다.", server_addr);

    // 메인 스레드 loop 문 --- (*5)
    loop {
        // 클라이언트 접속 처리 --- (*6)
        if let Ok((client, addr)) = server.accept() {
            println!("클라이언트 접속: {}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }
        // 스레드간 통신 대기 --- (*7)
        if let Ok(msg) = rx.try_recv() {
            println!("전원에게 보내기 : {}", msg.trim());
            clients = send_all(clients, &msg);
        }
        thread::sleep(Duration::from_millis(100));
    }
}
// 클라이언트가 보내는 메시지 수신 스레드 --- (*8)
fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        // 메시지 수신 대기 -- (*9)
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            // 수신한 메시지를 메인 스레드로 전달 --- (*10)
            if n > 0 {
                tx.send(msg).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}
// 모든 클라이언트에게 메시지 전송 --- (*11)
fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];
    for mut socket in clients.into_iter() {
        // 문자열을 바이트열로 변환해 전송 --- (*12)
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("전송 에러 : {}", e);
            continue;
        }
        collector.push(socket); // 소유권 회수
    }
    collector // 소유권 반환
}
