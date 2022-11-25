use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::Deserialize;

// 서버 주소와 포트 지정 --- (*1)
const SERVER_ADDR: &str = "127.0.0.1:8888";

// Actix Web 메인 함수 --- (*2)
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[SERVER] http://{}/", SERVER_ADDR);
    // 서버 기동 --- (*3)
    HttpServer::new(|| {
        // 라우팅 지정 --- (*4)
        App::new()
            .service(index)
            .service(calc)
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}

// 서버 루트. 키와 몸무게를 입력받을 페이지 --- (*5)
#[get("/")]
async fn index(_: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(r#"
            <html><body><h1>BMI 계산 및 비만도 판정</h1>
            <form action='calc'>
            <div>키: <div><label><input name='height' value='160'></label></div></div>
            <div>몸무게: <div><label><input name='weight' value='70'></label></div></div>
            <div><label><input type='submit' value='확인'></label></div>
            </form></body></html>"#
        ))
}

// 입력 폼 데이터 정의 --- (*6)
#[derive(Deserialize, Debug)]
pub struct FormBMI {
    height: f64,
    weight: f64,
}

// 실제로 BMI를 계산해 표시해줄 페이지 --- (*7)
#[get("/calc")]
async fn calc(q: web::Query<FormBMI>) -> Result<HttpResponse, Error> {
    // 폼으로 전달받은 매개변수 확인용(CLI)
    println!("{:?}", q);
    // BMI 계산
    let h = q.height / 100.0;
    let bmi = q.weight / (h * h);
    let per = (bmi / 22.0) * 100.0;
    // 결과 표시
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<h3>BMI = {:.1}, 비만율 = {:.0}%</h3>", bmi, per)))
}
