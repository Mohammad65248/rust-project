fn main() {
    let app_name = "Rust Learning Project";
    let version = 1;
    let is_active = true;

    println!("Welcome to {} (Version {})", app_name, version);
    println!("Status Active: {}", is_active);

    let result = add_numbers(5, 10);
    println!("5 + 10 ka result hai: {}", result);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
