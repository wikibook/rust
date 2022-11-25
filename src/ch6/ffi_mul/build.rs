extern crate cc;

fn main() {
    // C 언어 소스 코드 컴파일
    cc::Build::new()
        .file("src/mycalc.c") // C 언어 소스 코드 파일 지정
        .include("src")
        .compile("mycalc"); // 출력할 라이브러리 이름 지정
}
