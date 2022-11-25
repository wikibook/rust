use std::rc::Rc;
fn main() {
    // 힙 영역에 i32 타입 값 저장 --- (*1)
    let a_rc = Rc::new(1000);
    {
        // i32 타입을 참조하는 b_rc도 생성 --- (*2)
        let b_rc = Rc::clone(&a_rc);
        println!("{}", b_rc);
        // a_rc의 참조 카운터 확인 --- (*3)
        println!("참조 카운트 = {}", Rc::strong_count(&a_rc));
    } // a_rc의 참조 카운트가 1 감소
    println!("{}", a_rc); // Rc 타입이므로 이용 가능 --- (*4)
    println!("참조 카운트 = {}", Rc::strong_count(&a_rc));
}