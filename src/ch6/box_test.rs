fn main() {
    // 힙 메모리에 100을 저장하고 포인터를 반환
    let x_box = Box::new(100);
    // 역참조를 해서 원래의 값을 가져온다
    let x_val = *x_box;
    println!("{}", x_val);
}