pub struct Node {
    data: i64,
    link: Option<Box<Node>>
}

fn node(v: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node{data:v, link:link}))
}
fn main() {
    let mut c1 = node(10, None).unwrap();
    let mut c2 = node(20, None).unwrap();
    let mut c3 = node(30, None).unwrap();

    c2.link = Some(c3);
    c1.link = Some(c2);
    let c = c1;

    let mut p = &c;
    loop {
        println!("{}", p.data);
        match p.link {
            None => break,
            Some(ref link) => p = &link,
        }
    }
}

