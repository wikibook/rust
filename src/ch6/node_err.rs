use std::rc::Rc;
use std::cell::RefCell;
// 양방향 리스트 정의(순환 참조 문제 발생)
struct Node {
    data: isize,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    // 값 생성
    let a = Rc::new(RefCell::new(
            Node{data:10, prev:None, next:None}));
    let b = Rc::new(RefCell::new(
            Node{data:20, prev:None, next:None}));
    // a와b를 서로 참조
    b.borrow_mut().prev = Some(Rc::clone(&a));
    a.borrow_mut().next = Some(Rc::clone(&b));
    // 참조 카운트 확인
    println!("a: {}", Rc::strong_count(&a));
    println!("b: {}", Rc::strong_count(&b));
    println!("{}", a.borrow().data);
}

