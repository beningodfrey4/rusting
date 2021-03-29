fn main() {
    let mut x = String::from("what am I doing");
    x.push(' ');

    let y = please_dont_go(x);
    // println!("{}", x);
    println!("{}", y);

    println!("{}", come_back(&mut String::from("Sending literally nothing"), &mut String::from("more nothingness")));
}

fn please_dont_go(mut x: String) -> usize {
    x.push_str("with my life");
    x.len()
}

// fn come_back(x: &String, y: &String) -> &String {
fn come_back<'a>(x: &'a mut String, y: &'a mut String) -> &'a mut String {
    y.push_str(&x);
    let x = y;
    x
}