// 보물 상자의 동작을 정의하는 트레잇 --- (*1)
trait TreasureBox {
    fn open(&self, key_no: i32) -> bool;
    fn check(&self);
}

// 보석이 든 상자의 구조체를 정의 --- (*2)
struct JewelryBox {
    price: i32,  // 몇 골드가 있는가
    key_no: i32, // 몇 번째 열쇠가 있어야 열리는가
}
impl TreasureBox for JewelryBox {
    fn open(&self, key_no: i32) -> bool {
        // 지정한 열쇠로만 상자가 열림
        self.key_no == key_no
    }
    fn check(&self) {
        println!("보석 상자였다. {} 골드 입수.", self.price);
    }
}

// 함정 상자의 구조체를 정의 --- (*3)
struct TrapBox {
    damage: i32,
}
impl TreasureBox for TrapBox {
    fn open(&self, _key: i32) -> bool {
        return true; // 어떤 키를 가지고 있어도 열림
    }
    fn check(&self) {
        println!("함정이었다. HP가 {} 감소했다.", self.damage);
    }
}

// 모험가가 상자를 여는 동작 --- (*4)
fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("열쇠가 맞지 않아 상자가 열리지 않는다.");
        return;
    }
    tbox.check();
}

fn main() {
    // 다양한 상자를 준비 --- (*5)
    let box1 = JewelryBox {
        price: 30,
        key_no: 1,
    };
    let box2 = TrapBox { damage: 3 };
    let box3 = JewelryBox {
        price: 20,
        key_no: 2,
    };
    // 모험가가 가진 열쇠로 상자를 연다 --- (*6)
    let my_key = 2;
    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}
