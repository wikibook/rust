fn main() {
    for i in 1..=100 {
        // 값을 match로 분기. 튜플을 이용한다. --- (*1)
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"), // --- (*2)
            (0, _) => println!("Fizz"), // --- (*3)
            (_, 0) => println!("Buzz"),
            _      => println!("{}", i),
        }
    }
}
