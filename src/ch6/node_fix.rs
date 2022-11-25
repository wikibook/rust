use std::rc::{Rc,Weak};
use std::cell::RefCell;
// 양방향 리스트 정의
struct Node {
    data: isize,
    prev: Option<Weak<RefCell<Node>>>, // 약한 참조
    next: Option<Rc<RefCell<Node>>>, // 강한 참조
}
fn main() {
    // 값 생성
    let a = Rc::new(RefCell::new(
            Node{data:10, prev:None, next:None}));
    let b = Rc::new(RefCell::new(
            Node{data:20, prev:None, next:None}));
    // a와b를 서로 참조
    a.borrow_mut().next = Some(Rc::clone(&b));
    b.borrow_mut().prev = Some(Rc::downgrade(&a));
    // 참조 카운트 확인
    println!("a: {}", Rc::strong_count(&a));
    println!("b: {}", Rc::strong_count(&b));
    // 값 출력
    println!("b.data= {}", b.borrow().data);
    // b에서a의 값 얻기
    match &b.borrow().prev {
        None => {},
        Some(prev) => {
            // 
            let pa = prev.upgrade().unwrap();
            println!("a.data= {}", pa.borrow().data);
        },
    };
}

