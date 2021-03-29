struct node {
    key: i32,
    left: Option<&node>,
    right: Option<&node>,
}

fn main() {
    let root = Some(node{key: 10, left: None, right: None});
}

fn empty(x: Option<node>) -> bool {
    match x { // consider a match as a control flow operator acting on Option types
        Some(k) => {
            println!("{} is root", k.key);
            return true;
        }
        None => {
            println!("Empty");
            return false;
        }
    }
}
