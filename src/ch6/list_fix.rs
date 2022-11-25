// 단방향 연결 리스트 --- (*1)
pub struct Node {
    data: i64,
    link: Option<Box<Node>>,
}
// 단방향 연결 리스트를 생성하는 함수 --- (*2)
fn node(v: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node {
        data: v,
        link: link,
    }))
}
fn main() {
    // 단방향 연결 리스트 생성 --- (*3)
    let c = node(10, node(20, node(30, None))).unwrap();

    // 가장 앞에서부터 각 요소를 따라가며 값을 표시 --- (*4)
    let mut p = &c;
    loop {
        println!("{}", p.data);
        // p가 다음 요소를 가리키도록 변경 --- (*5)
        match p.link {
            None => break,
            Some(ref link) => p = &link,
        }
    }
}
