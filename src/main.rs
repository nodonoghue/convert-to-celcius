use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a temp in F to convert to C");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Please type a number!");
    let c_temp = convert_to_celcius(input);
    println!("{}F is {}C", input, c_temp);
}

fn convert_to_celcius(temp: i32) -> i32 {
    (((temp as f32) - 32.0) * (5.0 / 9.0)) as i32
}
