fn main () {
    let mut x = 5;
    println! ("The value of x is {}", x);
    x = 6;
    println! ("The value of x is {}", x);
    const _MAX: u32 = 10000; // works only with compile time type-annotated constants. Basically constexpr from C++
    let _x = x + 1;
    let _x = _x * 2;    // let, though default immutable, gives type conversion and rebinding with same identifier
    let mut _spaces = "   ";
    // _spaces = _spaces.len ();   //mut wouldn't work here as you cannot mutate a variable's type
    let _y = func (10);
}

fn func (x: u32) -> u32 {
    x + 2
}
