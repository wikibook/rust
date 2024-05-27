fn main() {
    let moon: f32 = 384400.0;
    let car: f32 = 80.0;
    let btrain: f32 = 300.0;
    println!("달까지 자동차로 {}일", moon / car / 24.0);
    println!("달까지 KTX로 {}일", moon / btrain / 24.0);
}

