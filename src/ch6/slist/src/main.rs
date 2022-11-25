mod slist;
fn main() {
    // 리스트 객체 생성
    let mut list = slist::List::new();
    // 리스트의 끝에 값을 추가
    list.push(100);
    list.push(200);
    // 리스트의 앞에 값을 추가
    list.unshift(10);
    list.unshift(20);
    // 지정한 인덱스에서 값 가져오기
    println!("{}", list.get(0).unwrap());
    println!("{}", list.get(1).unwrap());
    println!("{}", list.get(2).unwrap());
    println!("{}", list.get(3).unwrap());
}
