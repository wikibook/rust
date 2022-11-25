fn main() {
    let r = 3..15;
    println!("{}..{}", r.start, r.end);
    for a in r {
        println!("{}", a);
    }
}
