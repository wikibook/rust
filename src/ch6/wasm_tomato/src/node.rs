// 문법 요소를 Node 라는 타입으로 정의 --- (*1)
#[derive(Debug, Clone)]
pub enum Node {
    Nop, // 아무것도 하지 않음
    Number(i64), // 숫자 값을 나타냄
    Calc(char, Box<Node>, Box<Node>), // 계산식
    If(Box<Node>, Box<Vec<Node>>, Box<Vec<Node>>), // if문
    For(String, i64, i64, Box<Vec<Node>>), // for문
    Print(Box<Node>), // print문(계산 출력)
    PrintStr(String), // print문(상수 출력)
    SetVar(String, Box<Node>), // 변수 대입
    GetVar(String), // 변수 참조
}
impl Node {
    // Node::Calc 타입을 반환하는 함수 --- (*2)
    pub fn calc(op: char, l: Node, r: Node) -> Node {
        Node::Calc(op, Box::new(l), Box::new(r))
    }
    // Node::If 타입을 반환하는 함수 --- (*3)
    pub fn if_(cond: Node, t: Vec<Node>, f: Vec<Node>) -> Node {
        Node::If(Box::new(cond), Box::new(t), Box::new(f))
    }
}
