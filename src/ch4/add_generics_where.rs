fn add <T> (a:T, b:T) -> T
    where T: std::ops::Add<Output=T>
{
    a + b
}

fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
}