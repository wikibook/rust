use tide::prelude::*;
const SERVER_ADDR: &str = "127.0.0.1:8888";

// 이름 정보를 표시할 구조체 정의 --- (*1)
#[derive(Deserialize, Serialize)]
struct UserInfo {
    name: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("http://{}/", SERVER_ADDR);
    let mut app = tide::new();
    // 기본 페이지에 접속할 때의 처리 --- (*2)
    app.at("/").get(|_| async { // 이 경로에 접속할 때
        // 다음 HTML 코드를 출력
        Ok(tide::Response::builder(200)
            .content_type(tide::http::mime::HTML)
            .body(r#"
                <html><body><form action='hello'>
                name: <input name='name' value='남방큰돌고래'>
                <input type='submit' value='전송'>
                </form></body></html>"#)
            .build())
    });
    // "/hello"에 접속할 때의 처리 --- (*3)
    app.at("/hello").get(|req: tide::Request<()>| async move {
        // 전송받은 데이터를 구조체에 할당 --- (*4)
        let user: UserInfo = req.query()?;
        Ok(tide::Response::builder(200)
           .content_type(tide::http::mime::HTML)
           .body(format!("<h1>안녕하세요, {}님</h1>", user.name))
           .build())
    });
    // 서버 실행
    app.listen(SERVER_ADDR).await?;
    Ok(())
}

