fn main() {
    // 블록
    {
        let s1 = String::from("재능은 한계가 있지만 노력엔 한계가 없다");
        println!("{}", s1);
    }
    // 블록을 벗어나면 s1은 파기된다
}