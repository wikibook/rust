// 단방향 연결 리스트에서 사용할 한 요소의 구조체 --- (*1)
pub struct Node {
    data: isize,
    link: Option<Box<Node>>,
}
// 단방향 연결 리스트 자체의 구조체  --- (*2)
pub struct List {
    head: Option<Box<Node>>,
}
// List 구조체의 메서드 구현 --- (*3)
impl List {
    pub fn new() -> Self { // 생성자
        Self{head: None}
    }
    // 리스트의 제일 앞에 값을 추가 --- (*4)
    pub fn unshift(&mut self, v:isize) {
        let new_node = Node{data: v, link: self.head.take()};
        self.head = Some(Box::new(new_node));
    }
    // 리스트의 제일 끝에 값을 추가 --- (*5)
    pub fn push(&mut self, v:isize) {
        // 새로운 값
        let new_node = Node{data: v, link: None};
        match self.head {
            None => self.head = Some(Box::new(new_node)),
            Some(ref mut head) => {
                // 현재의 가장 끝 노드를 찾아 새로운 노드에 연결
                let mut p = head;
                loop {
                    match p.link {
                        None => { // 가장 끝 노드
                            p.link = Some(Box::new(new_node));
                            break;
                        },
                        Some(ref mut next) => p = next,
                    }
                }
            }
        }
    }
    // 지정한 인덱스의 값을 가져오기 --- (*6)
    pub fn get(&self, index: isize) -> Option<isize> {
        match self.head {
            None => return None, // 리스트가 비어있을 때
            Some(ref top) => {
                // 지정한 인덱스 값 찾기
                let mut p = top;
                let mut i = 0;
                loop {
                    if i == index { // 찾았을 때
                        return Some(p.data);
                    }
                    match p.link { // 다음 요소 찾기
                        None => return None,
                        Some(ref link) => p = link,
                    }
                    i += 1;
                }
            }
        }
    }
}