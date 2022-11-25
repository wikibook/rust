fn main() {
    let s = "내 자신에 대한 자신감을 잃으면 온 세상이 나의 적이 된다.";
    let s = s.replace("잃으면", "가지면");
    let s = s.replace("적이", "편이");
    println!("{}", s);
}