fn main() {
    // Integers

    let a = 98_2222; // Decimal
    let b = 0xff; // Hexadecimal
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8)

    let f: u8 = 255; // Explicit type annotation

    // Floating-point numbers
    let g: f32 = 2.0; // f32

    //addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f = false;

    // Characters
    let c1 = 'z';
    let c2 = '😻'; // Unicode character

    // Compound types
    // Tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Arrays = fixed-size
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // Vectors = growable arrays
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
}
