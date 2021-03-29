fn main() {
    let a = String::from("Hello, world");
    let b = a;
    //println!("{}", a);    implicit move as String has no Copy trait implemented as it is heap allocated
    println!("{}", b);
    
    let c = b.clone();      //explicit copy of heap allocated String

    let c = 5;
    led d = c;  //allowed as types that implement the Copy trait are implicitly copied(here int is small and stack-allocated, so copying implicitly is fine)
    
    pass_by_value_implies_move(c);      //if copy trait is not implemented
    //println!("{}", c);    //moved by semantic as above, dropped at end of owned scope

    let a = returning_ownership();      //note reuse of a
}

fn pass_by_value_implies_move(s: String) {
    println!("{}", s.capacity());
    //s dropped here as ownership was transferred to it
}

fn returning_ownership() -> String {
    let s = String::new();
    return s;
}
