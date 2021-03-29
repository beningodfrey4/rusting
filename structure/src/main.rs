#[derive(Debug)]

struct Rectangle {
    length: u32,
    breadth: u32,
}

impl Rectangle {
    fn contains(&self, r: &Rectangle) -> bool {
        /*if (self.length >= r.length) && (self.breadth >= r.breadth) {
            return true;
        }
        return false;*/
        (self.length >= r.length) && (self.breadth >= r.breadth)
    }
}

fn main() {
    let a = Rectangle{length: 10, breadth: 5};
    let b = Rectangle{length: 4, breadth: 3};
    println!("{}",a.contains(&b));
    println!("{:?} \n{:?}", a, b);
}
