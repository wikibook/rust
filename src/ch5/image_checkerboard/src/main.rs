use image::{Rgb, ImageBuffer};
fn main() {
    // RGB 코드로 흰색 지정 --- (*1)
    let white = Rgb::<u8>([255, 255, 255]);
    // RGB 코드로 붉은 색을 지정
    let red = Rgb::<u8>([255, 90, 90]);
    // 1칸의 크기(픽셀)
    let w = 64;
    // 체크무늬를 그리는 클로저 --- (*2)
    let draw = |x, y| {
        let (xi, yi) = (x / w, y / w);
        match (xi % 2, yi % 2) {
            (0, 0) => white,
            (1, 0) => red,
            (0, 1) => red,
            (1, 1) => white,
            (_, _) => panic!("error"),
        }
    };
    // 클로저를 지정해 ImageBuffer를 생성 --- (*3)
    let img = ImageBuffer::from_fn(512, 512, draw);
    // 파일 저장 --- (*4)
    img.save("checkerboard.png").unwrap();
}
