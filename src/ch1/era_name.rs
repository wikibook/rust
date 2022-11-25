fn main() {
    for y in 1392..1451 {
        // 서력을 출력(행을 바꾸지 않음)
        print!("서력 {} 년 = ", y);
        // 대응하는 조선 연호를 출력 후 행을 바꿈
        if y >= 1419 {
            if y == 1419 { println!("세종 원년"); }
            else { println!("세종 {} 년", y-1419+1); }
        } else if y >= 1401 {
            if y == 1401 { println!("태종 원년"); }
            else { println!("태종 {} 년", y-1401+1); }
        } else if y >= 1399 {
            if y == 1399 { println!("정종 원년"); }
            else { println!("정종 {} 년", y-1399+1); }
        } else if y >= 1392 {
            if y == 1392 { println!("태조 원년"); }
            else { println!("태조 {} 년", y-1392+1); }
        }
    }
}

