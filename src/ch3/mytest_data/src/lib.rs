#[cfg(test)]
mod tests {
    #[test]
    fn array_test() {
        // 숫자 값 배열 생성 및 비교 --- (*1)
        let a1 = [100, 200, 300];
        let a2 = [100, 200, 300];
        assert_eq!(a1, a2);
        // String 배열 생성 및 비교 --- (*2)
        let a3 = [
            "사과".to_string(), 
            "바나나".to_string()];
        let a4 = [
            String::from("사과"), 
            String::from("바나나")];
        assert_eq!(a3, a4);
    }
    #[test]
    fn vec_test() {
        // 벡터(Vec<&str>) 생성 및 비교 --- (*3)
        let v1 = vec!["apple", "banana", "mango"];
        let mut v2:Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banana");
        v2.push("mango");
        assert_eq!(v1, v2);
    }
}
