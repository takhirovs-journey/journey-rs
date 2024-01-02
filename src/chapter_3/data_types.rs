/// # Scalar types:
///
/// A scalar type represents a single value.
/// In Rust has a four scalar types:
/// [integers, floating-point, boolean, character]
///
/// ## Integer types:
/// An integer types is a number without fractional components.
/// We can use any of these variants to declare the type of an integer value.
/// -------------------------------
/// | Length  | Signed | Unsigned |
/// |---------|--------|----------|
/// | 8-bit   | i8     | u8       |
/// | 16-bit  | i16    | u16      |
/// | 32-bit  | i32    | u32      |
/// | 64-bit  | i64    | u64      |
/// | 128-bit | i128   | u128     |
/// | arch    | isize  | usize    |
/// -------------------------------
///
/// ## Floating-point (float) types:
/// In floating-point types has two primitive types,
/// which are numbers with decimal points.
/// Floating-point types are `f32` and `f64`.
///
/// ## Boolean types:
/// A Boolean type in Rust has two possible values: `true` and `false`.
/// Booleans are one byte in size. The Boolean type in Rust is specified using bool
///
/// ## Character type:
/// Rust's `char` type is four bytes in size and represents a Unicode Value,
/// which means it can represent a lot more than just ASCII
///
/// ! Note: that we specify char literals with single quotes,
/// as opposed to string literals, which use double quotes.
fn scalar_types() {
    // Integer
    let _integer: u8 = 90;
    let _integer = 90_u8;
    println!("Integer: {}", _integer);

    // Floating-point (fractional integer)
    let _float: f32 = 9.99;
    let _float = 9.99_f32;
    println!("Floating-point: {}", _float);

    // Booleans
    let _boolean: bool = true;
    let _boolean: bool = false;
    println!("Boolean: {}", _boolean);

    // Characters
    let _character: char = 'a';
    let _character: char = 'Z';
    println!("Character: {}", _character);
}

/// # Compound Types:
///
/// Compound types can group multiple values into one type.
/// Rust has two primitive compound types: [tuples, arrays]
///
/// ## Tuples:
/// A tuple is a general way of grouping together a number of values
/// with a variety of types into one compound type. Tuples have a fixed length:
/// once declared, they cannot grow or shrink in size.
///
/// Each position in the tuple has a type, and the types of the different values
/// in the tuple don’t have to be the same. We can added optional type annotations.
///
/// ## Array:
/// Arrays are another way to store data, but unlike tuples,
/// all elements in arrays must be of the same type.
///
/// Arrays are useful when we want our data allocated on the stack rather than the heap,
/// or when we want to ensure we always have a fixed number of elements.
///
/// ! Note: Arrays are non-growing, that is, unlike other programming languages,
/// arrays in Rust are static, meaning that an array cannot be grown or extended.
fn compound_types() {
    // Tuples
    let tuple_1: (u8, f32, char) = (10, 25.5, 'W');
    let tuple_2: (&str, u8) = ("Salom", 20);

    // We can access a tuple element by using dot (.)
    println!("Tuple #1: ({}, {}, {})", tuple_1.0, tuple_1.1, tuple_1.2);
    println!("Tuple #2: ({}, {})", tuple_2.0, tuple_2.1);

    // Take tuple and turn into two separate variables.
    // This is called `destructuring`, because it breaks the single tuple into two parts
    let (greeting, some_num) = tuple_2;
    println!("Greeting msg: {}", greeting);
    println!("Number: {}", some_num);

    // Arrays
    #[rustfmt::skip]
    let months: [&str; 12] = [ // That means we want 12 string elements in this array
        "January", "February", "March",
        "April", "May", "June",
        "July", "August", "September",
        "October", "November", "December",
    ];

    // We can access an array using indexed numbers
    // The Rust compiler gives us a panic when an array is given an invalid index number
    println!("It is now {}", months[0]); // <- The number `0` represents the 1th element in the array
}

#[allow(unused)]
pub fn main() {
    println!("| Data types:");

    println!("|| Scalar types:");
    scalar_types();
    println!("-------------------------");

    println!("|| Compound types:");
    compound_types();
    println!("-------------------------");
}
