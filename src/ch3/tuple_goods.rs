fn main() {
    // 튜플 만들기 --- (*1)
    let banana = ("바나나", 300);
    let apple = ("사과", 200);
    // 튜플을 참조해 합계 금액 구하기 --- (*2)
    let total = banana.1 + apple.1;
    // 튜플 내용 표시 --- (*3)
    print_tuple(&banana); 
    print_tuple(&apple);
    println!("합계는 {}원 입니다.", total);
}
// (상품 이름, 금액) 튜플을 표시하는 함수 --- (*4)
fn print_tuple(item: &(&str, i64)) {
    println!("{}를 {}원에 구입.", item.0, item.1);
}