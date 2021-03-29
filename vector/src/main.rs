fn main() {
    let v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100); // returns an Option type, some if in in range and None if not. Exhaustive checking by match can be performed.

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
