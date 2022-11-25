// 구조체 Point 정의 --- (*1)
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 메서드 정의 --- (*2)
impl<T> Point<T> where T: std::ops::AddAssign {
    // 생성자 --- (*3)
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    // 값 더하기 --- (*4)
    fn add(&mut self, pt: Point<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}
fn main() {
    // Point 객체 생성
    let mut pt = Point::new(10, 10);
    println!("{:?}", pt);
    // 좌표 값 더하기
    pt.add(Point{ x:20, y:30 });
    println!("{:?}", pt);
}