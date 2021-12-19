// vector code
pub fn run() {
    // type  can't be inferred because no values are passed
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    // vec! is a macro
    // vectors and their ontents get dropped when they go out of scope
    let _v2 = vec![1, 2, 3];

    let third = &v[2];
    println!("Third is: {}", third);

    // the get method accesses vectors without crashing if given an invalid i
    match v.get(20) {
    Some(third) => println!("Third is: {}", third),
    None => println!("There is no third element")
    }

    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    match &row[1] {
        &SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer!")
    }
}
