fn main() {
    let s = String::from("hello world");

    let _hello = &s[0..5]; // follows same rules of references

    let words = String::from("here are some words");

    let word = first_word(&words);

    println!("{}", word)

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
