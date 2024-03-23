
fn integer_type() {
    // BY DEFAULT THE INTEGER TYPE IS i32
    //Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1
    let a: i8 = 127; 
    let b: i16 = 32767;
    let c: i32 = 2147483647;
    let d: i64 = 9223372036854775807;
    let e: i128 = 170141183460469231731687303715884105727;
    let f: isize = 9223372036854775807;
    //Unsigned variants can store numbers from 0 to 2n - 1,
    let g: u8 = 255;
    let h: u16 = 65535;
    let i: u32 = 4294967295;
    let j: u64 = 18446744073709551615;
    let k: u128 = 340282366920938463463374607431768211455;
    let l: usize = 18446744073709551615;

    println!("Signed integers: {}, {}, {}, {}, {}, {}", a, b, c, d, e, f);
    println!("Unsigned integers: {}, {}, {}, {}, {}, {}", g, h, i, j, k, l);
}
fn float_type() {
    // BY DEFAULT THE FLOAT TYPE IS f64
    // float types don't have usigned type
    let a: f32 = 3.14;
    let b: f64 = 3.141592653589793;
    println!("Floating point numbers: {}, {}", a, b);
}
fn boolean_type(){
    let t: bool = true;
    let f: bool = false;
    println!("Boolean type: {}, {}", t, f);

}
fn character_type(){
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value,
    // which means it can represent a lot more than just ASCII.
    // Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces 
    //are all valid char values in Rust
    let c: char = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat: char = '😻';
    println!("Character type: {}, {}, {}", c, z, heart_eyed_cat);

}
fn tuple_type() {
    // Tuples can hold values of different types
    let tuple: (i32, f64, char) = (42, 3.14, 'a');
    println!("Tuple type: {:?}", tuple);
    // Accessing tuple elements
    let first = tuple.0;
    let second = tuple.1;
    let third = tuple.2;
    println!("Tuple elements: {}, {}, {}", first, second, third);
}

fn array_type(){
    // Arrays in Rust have a fixed length, like tuples
    // The type of an array is written as [T; N], where T is the type of the elements and N is a compile-time constant
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // this is the same as let c = [3, 3, 3, 3, 3];
    println!("Array type: {:?}, {:?}, {:?}", a, b, c);
    let first = a[0];
    let second = a[1];
    println!("Array elements: {}, {}", first, second);

}



fn main() {
    integer_type();
    float_type();
    tuple_type();
    array_type();
    character_type();
    boolean_type();
}
