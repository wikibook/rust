fn main() {
    let g1 = String::from("실수할 줄 아는 사람이 아름답다");
    show_message(g1); // 소유권이 이동한다
    println!("{}", g1); // g1은 사용할 수 없다
}

fn show_message(message: String) {
    println!("{}", message);
}