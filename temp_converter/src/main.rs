fn main() {
    let temp = 40;

    println!("{} degree celsius = {} fahrenheit", temp, fahrenheit(temp));
    println!("{} degree fahrenheit = {} celsius", temp, celsius(temp));
}

fn celsius(fahrenheit: i32) -> i32 {
    return (fahrenheit - 32) * 5 / 9;
}

fn fahrenheit(celsius: i32) -> i32 {
    return (celsius * 9 / 5) + 32;
}
