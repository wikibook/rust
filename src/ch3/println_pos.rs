fn main() {
    let year = 2023;
    let month = 12;
    let day = 1;
    println!("KR:{0}/{1}/{2}", year, month, day); // 한국
    println!("US:{1}/{2}/{0}", year, month, day); // 미국
    println!("UK:{2}/{1}/{0}", year, month, day); // 영국
}