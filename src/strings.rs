use unicode_segmentation::UnicodeSegmentation;

// String code
pub fn run() {
    // Strings are stored as a collection of UTF-8 encoded bytes
    let _s1 = String::new();
    let s2 = "initial contents:";
    let _s3 = s2.to_string();
    let _s4 = String::from(s2);

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    // s7 and s8 do the same thing
    let _s7: String = s5 + &s6;
    // format doesn't take ownership
    let _s8 = format!("{}{}", "Hello, ", s6);

    let hello: String = String::from("Hello");
    // let c: char = hello[0];
    // the above line errors because each character is not garaunteed 1 byte
    // in unicode

    // access as bytes
    for b in hello.bytes() {
        print!("{} ", b);
    }
    println!();

    // access as scalars
    for c in hello.chars() {
        print!("{}", c);
    }
    println!();

    // access as Graphemes
    // requires inporting unicode segmentation in cargo.toml
    for g in hello.graphemes(true) {
        print!("{}", g);
    }
    println!();
}
