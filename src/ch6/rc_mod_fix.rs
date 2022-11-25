use std::rc::Rc;
use std::cell::RefCell;
fn main() {
    // 힙 영역 안에 가변형 i32 타입 값 1000을 저장 --- (*1)
    let a = Rc::new(RefCell::new(1000));
    // 참조 카운터 증가 --- (*2)
    let b = Rc::clone(&a);
    // 값 변경 --- (*3)
    *b.borrow_mut() += 100;
    // 변경된 값 출력 --- (*4)
    println!("{}", a.borrow());
}