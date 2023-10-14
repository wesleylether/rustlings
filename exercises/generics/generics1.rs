// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    // you can also use Vec<_> so rust can figure it out itself
    // But in this example you don't need a type because,
    // you push values in it and rust wil automatically know what type it is
    shopping_list.push("milk");
}
