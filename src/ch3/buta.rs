fn main() {
    let pr = "위키북스";
    // 1바이트씩 표시 --- (*1)
    for c in pr.bytes() {
        print!("{:2x} ", c);
    }
}