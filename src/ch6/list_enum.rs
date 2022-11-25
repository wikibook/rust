// 열거형으로 Node를 정의 --- (*1)
enum Node {
    Empty,
    Cons(i64, Box<Node>),
}
// 열거형을 더 쉽게 사용할 수 있게 해주는 선언 및 함수 --- (*2)
use Node::{Empty, Cons};
fn node(v: i64, link: Box<Node>) -> Box<Node> {
    Box::new(Cons(v, link))
}
fn main() {
    // 단방향 연결 리스트 생성 --- (*3)
    let c = node(10, node(20, node(30, Box::new(Empty))));

    // 가장 앞에서부터 각 요소를 따라가며 값을 표시 --- (*4)
    let mut ptr: &Box<Node> = &c;
    loop {
        // &Box<Node>에서 Node를 꺼내와 출력 --- (*5)
        let cur_node: &Node = &**ptr; 
        match cur_node {
            Empty => break,
            Cons(v, link) => {
                println!("{}", v);
                ptr = &link;
            }
        }
    }
}