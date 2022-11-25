use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};

// 서버 주소와 포트를 지정 --- (*1)
const SERVER_ADDR: &str = "127.0.0.1:8888";

// Actix Web 메인 함수 --- (*2)
#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    println!("[SERVER] http://{}/", SERVER_ADDR);
    // HTTP 서버 시작 --- (*3)
    HttpServer::new(|| {
        // 라우팅 지정 --- (*4)
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(SERVER_ADDR)? 
    .run()
    .await?;
    Ok(())
}

// 실행할 함수 --- (*5)
async fn index(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    println!("request: {:?}", req);
    let result = "Hello, World!";
    Ok(HttpResponse::Ok().body(result))
}
