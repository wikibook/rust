fn main() {
    let target = "aaa,bbb,ccc";
    // for 문을 사용해 Vec<String> 으로 변환
    let mut lines = vec![];
    for line in target.split(",") {
        lines.push(line.to_string());
    }
    println!("{:?}", lines);
    // map을 사용해 Vec<String> 으로 변환
    let lines:Vec<String> = 
        target.split(",").map(|s| s.to_string()).collect();
    println!("{:?}", lines);
}

