fn main() {
    let variable = 10;
    println!("The value of the immutable variable is: {}", variable);
    let mut mut_variable = 10;
    println!("The value of the mutable variable is: {}", mut_variable);
    mut_variable += 5;
    println!("value of the mutable variable after modification is: {}", mut_variable);
    let shadowed_variable = 20;
    println!("The value of the shadowed variable is: {}", shadowed_variable);
    let shadowed_variable = shadowed_variable + 5;
    println!("shadowed variable after modification is: {}", shadowed_variable);
    {
        let shadowed_variable = shadowed_variable + 30;
        println!("The value of the shadowed variable in the inner scope is: {}", shadowed_variable);
    }
    println!("The value of the shadowed variable in the outer scope is: {}", shadowed_variable);

}
