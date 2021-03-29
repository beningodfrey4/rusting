fn main() {
    let l = calculate_length(&String::from("test"));
    println!("{}", l);
    let mut x = String::from("test");
    change(&mut x); // no ownership transferred to change, so no move and drop, but a pointer to this reference is created locally.
    let _y = &x;
    let _z = &x;        //allowed as _y and _z are immutable
    //let _a = &mut x;  //cannot mix mutable and immutable references

    let mut s = String::from("this");
    let _t = &mut s;
    //let _u = &mut s;  //can't have multiple mutable borrows in the same scope

    let &mut b = returning_references();

    let a: &str = using_slices(&b);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(x: &mut String) {
    x.push_str(" this");
}

fn returning_references() -> &mut String {
    return &mut String::from("this is why I need rust");    //this is an error as ownership is not transferred
}       //to caller as it is a reference but the string is dropped as the scope of the string is over

fn using_slices(b: &str) -> &str {
    return &b[..];
}