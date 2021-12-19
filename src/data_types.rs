pub fn run() {
/********************************BASIC TYPES**********************************/    
    // Integers

    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)

    // setting f to higher than the maximum(256)
    // wraps back to the minimum in release builds
    let f: u8 = 255;

    // Floating-point
    let g = 2.0; // 64-bit(default)
    let h: f32 = 3.0; // 32-bit

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 -4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // mod
    let remainder = 43 % 5;

    // Booleans
    let t = true;

    let false_bool = false;

    // Character
    let character = 'z';

    println!("{:?} {:?}",
    (a, b, c, d, e, f, g, h, sum, difference, product, quotient),
    (remainder, t, false_bool, character));

/********************************BASIC TYPES**********************************/    

/******************************COMPOUND TYPES*********************************/

    // Tuples
    // Fixed size and can hold different types with up to 12 members
    let tup = ("Let's Get Rusty!", 100_000);
    // tuple destructuring; sets the corresponding field in tup to the named 
    // field on the left
    let (channel, sub_count) = tup;
    // fields can also be accessed using the dot notation with a field's index
    let sub_count1 = tup.1;

    println!("{:?}", (tup, channel, sub_count, sub_count1));

    // Arrays fixed length with members of the same type
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    // accessing a non-existant index creates an index out of bounds exception

    // arrays can also be declared in the form let name = [value; size]
    // this creates an array of size size with every member set to value
    let byte = [0; 8];

    println!("{:?}", (error_codes, not_found, byte));


/******************************COMPOUND TYPES*********************************/       
}
