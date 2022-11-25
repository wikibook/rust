// 서버 주소와 포트를 지정 --- (*1)
const SERVER_ADDR: &str = "127.0.0.1:8888";

// 메인 함수 --- (*2)
#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("http://{}/", SERVER_ADDR);
    // Tide 객체 생성 --- (*3)
    let mut app = tide::new();
    // 라우팅 지정 --- (*4)
    app.at("/").get(|_| async { Ok("Hello, World!") });
    // 서버 기동 --- (*5)
    app.listen(SERVER_ADDR).await?;
    Ok(())
}
