// 구조체 정의 --- (*1)
#[derive(Debug,PartialEq)]
struct GItem {
    name: String,
    price: i64,
}

#[cfg(test)]
mod tests {
    use super::*; // 모듈 밖의 요소를 이용한다는 것을 선언 --- (*2)
    #[test]
    fn item_test() {
        // 구조체 초기화 --- (*3)
        let apple1 = GItem{
            name: String::from("사과"),
            price: 2400,
        };
        let mut apple2 = GItem{
            name: "사과".to_string(),
            price: 0,
        };
        apple2.price = 2400;
        
        // 구조체 필드 비교 --- (*4)
        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);
        
        // 구조체 자체를 비교 --- (*5)
        assert_eq!(apple1, apple2);
    }
}
