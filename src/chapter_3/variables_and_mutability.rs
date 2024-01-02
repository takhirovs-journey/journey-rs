#[allow(unused)]
const FOUR_HOURS_IN_SECONDS: u32 = 60 * 60 * 4;

/// Mutable and Immutable:
///
/// Mutability:
/// Mutability refers to the ability of a variable to be modified
/// or changed after it has been initially assigned a value.
///
/// Immutability:
/// Immutability means that once a variable is assigned a value,
/// it cannot be changed. The value remains constant throughout its scope.
/// Immutability ensures that a value does not change unexpectedly, making code
/// more predictable and easier to reason about. It helps prevent accidental modifications.
fn mut_immut() {
    let mut x = 5;
    println!("x is {}", x);

    x = 6;
    println!("x changed to {}", x);
}

/// Shadowing:
///
/// Declaring a new variable with the same name as an existing variable in the same scope,
/// effectively hiding the outer variable. The new variable can have a different type or value.
/// This is useful when want to reuse the variable name for a different purpose.
fn shadowing() {
    let some_variable: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", some_variable);

    let some_variable: &str = "Rust!";
    println!("Hello, {}", some_variable);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("Spaces length: {}", spaces);
}

#[allow(unused)]
pub fn main() {
    println!("| Variables and Mutability:");

    println!("|| Immutability and Mutability:");
    mut_immut();
    println!("-------------------------");

    println!("|| Shadowing:");
    shadowing();
    println!("-------------------------");
}
