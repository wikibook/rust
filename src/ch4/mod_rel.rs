mod random {
    pub mod linear {
        pub fn rand() -> u32 { // --- (*a)
            1
        }
    }
    pub mod xorshift {
        pub fn rand() -> u32 { // --- (*b)
            // (*a) 의 함수를 호출
            super::linear::rand()
        }
    }
}

fn main() {
    // (*b) 의 함수를 호출
    println!("{}", random::xorshift::rand());
}
