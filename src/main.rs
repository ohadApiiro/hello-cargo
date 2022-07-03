use std::io::Write;
use std::fs;

fn foo() {
    println!("foo");
}


struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 1,
        height: 2
    };

    let area = rect.area();
    let w = rect.width;
    println!("{} {}", w, area);

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("/home/ohad/src/rust/hello_cargo/file.txt").expect("error");

    file.write(b"to append\n");

    foo();
    println!("Hello, world!");
}
