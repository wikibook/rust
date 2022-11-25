// 보물 상자의 동작을 정의하는 트레잇 --- (*1)
trait TreasureBox {
    // 기본 메서드 --- (*2)
    fn open(&self, key_no: i32) -> bool {
        // 열쇠 번호가 맞으면 열림
        self.get_key_no() == key_no
    }
    fn check(&self);
    fn get_key_no(&self) -> i32;
}

// 보석이 든 상자 정의 --- (*3)
struct JewelryBox {
    price: i32,
    key_no: i32,
}
impl TreasureBox for JewelryBox {
    fn check(&self) {
        println!("보석 상자였다. {} 골드 입수.", self.price);
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

// 빈 상자 --- (*4)
struct EmptyBox {
    key_no: i32,
}
impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("상자는 비어있다.");
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

// 모험가가 상자를 여는 동작 --- (*5)
fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("열쇠가 맞지 않아 상자가 열리지 않는다.");
        return;
    }
    tbox.check();
}

fn main() {
    // 상자 생성 --- (*6)
    let box1 = JewelryBox {
        price: 30,
        key_no: 1,
    };
    let box2 = EmptyBox { key_no: 1 };
    let box3 = JewelryBox {
        price: 50,
        key_no: 2,
    };
    // 상자 열기 --- (*7)
    open_box(&box1, 1);
    open_box(&box2, 1);
    open_box(&box3, 1);
}
