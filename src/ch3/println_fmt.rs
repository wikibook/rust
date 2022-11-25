fn main() {
    // 오른쪽 정렬 및 16진수 출력
    println!("|{:>8}| #{:06x}", "red", 0xFF0000);
    println!("|{:>8}| #{:06x}", "green", 0x00FF00);
    println!("|{:>8}| #{:06x}", "blue", 0x0000FF);
    // 디버그 출력
    println!("|{:>8}| RGB{:?}", "yellow", (255,255,0));
}