mod chapter_1;
mod chapter_3;
mod guessing_game;

fn main() {
    println!("Welcome to Rust Programming Language Book exercises!");

    println!("Chapter1 ----------------");
    // Hello World!
    chapter_1::main();

    // println!("Guessing game -----------");
    // guessing_game::main::main();

    println!("Chapter2 ----------------");
    // Immutability and Mutability
    chapter_3::variables_and_mutability::main();
    // Data types
    chapter_3::data_types::main();
    // Functions
    chapter_3::functions::main();
}
