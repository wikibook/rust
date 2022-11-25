mod aaa {
    pub mod bbb {
        pub mod ccc {
            // 함수 정의 --- (*1)
            pub fn print() {
                println!("aaa::bbb::ccc::print");
            }
        }
    }
    pub mod ddd {
        pub mod eee {
            // 함수 정의 --- (*2)
            pub fn print() {
                println!("aaa::ddd:eee:print");
            }
        }
        pub mod fff {
            // 함수 정의 --- (*3)
            pub fn print() {
                // 상대 경로로 함수 호출
                super::eee::print();
                super::super::bbb::ccc::print();
            }
        }
    }
}

fn main() {
    // 경로를 지정해 함수 호출 --- (*4)
    aaa::bbb::ccc::print();
    aaa::ddd::eee::print();
    // 최상위 경로부터 지정해 함수 호출 --- (*5)
    crate::aaa::ddd::fff::print();
}