fn my_function(value: i32, unit: char) -> i32 {
    println!("The measurement is: {value}{unit}");
    value + 3
}


fn main() {
    println!("Hello, world!");
    let x = my_function(5, 'h');
    println!("Returns: {x}");
}
