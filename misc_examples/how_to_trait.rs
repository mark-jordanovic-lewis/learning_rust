trait List {
    fn list(&self);
}

impl List for bool {
    fn list(&self) {
        println!("hello from bool");
    }
}

impl List for String {
    fn list(&self) {
        println!("hello from String");
    }
}

fn main() {
    let v: Vec<Box<List>> = vec![Box::new(true), Box::new(String::new())];

    match v.get(0) {
        None => panic!("just gonna panic here"),
        Some(s) => s.list()
    }

    match v.get(1) {
        None => panic!("just gonna panic here"),
        Some(s) => s.list()
    }
}
