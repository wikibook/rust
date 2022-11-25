// C 언어로 작성한 라이브러리의 이름 지정 --- (*1)
#[link(name="mycalc", kind="static")]
extern "C" {
    // C 언어에서 정의한 함수 지정
    fn mul(a: isize, b: isize) -> isize;
}

fn main() {
    // C 언어 함수 호출 --- (*2)
    unsafe {
        let n = mul(30, 5);
        println!("{}", n);
        let n = mul(8, 80);
        println!("{}", n);
    }
}