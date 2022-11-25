mod dlist;
fn main() {
    // 리스트 초기화
    let mut list = dlist::List::new();
    // 리스트의 끝에 값 추가
    list.push(100);
    list.push(110);
    // 리스트이 앞에 값 추가
    list.unshift(10);
    list.unshift(20);
    // 반복자로 값을 모두 표시
    for v in list.iter() {
        println!("{}", v);
    }
}
