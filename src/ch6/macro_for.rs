macro_rules! basic_for {
    // for i = 1 to 10 과 같이 작성하는 경우--- (*1)
    (
        for $i:ident = $from:tt to $to:tt
        $block:block
    ) => {{
        for $i in $from..=$to {
            $block
        }
    }};
    // for i = 1 to 10 step 2 와 같이 작성하는 경우 --- (*2)
    (
        for $i:ident = $from:tt to $to:tt step $step:tt
        $block:block
    ) => {{
        let mut $i = $from;
        loop {
            if $i > $to { break }
            $block
            $i += $step
        }
    }};
}

fn main() {
    // 매크로를 이용해 1부터 10까지의 합계 구하기 --- (*3)
    let mut total = 0;
    basic_for! {
        for i = 1 to 10 {
            total += i;
        }
    }
    println!("{}", total);
    // 매크로를 이용해 0부터 10까지 3이 증가할 때 마다 출력하기 --- (*4)
    basic_for! {
        for i = 0 to 10 step 3 {
            println!("i={}", i);
        }
    }
}