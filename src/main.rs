mod io;
mod temp;

fn main() {
    println!("\n[  Temperature converter  ] by znemux\n");
    println!("Is your temperature K, C or F?");
    let unit = io::input_char().to_ascii_uppercase();

    validate_unit(unit);

    println!("Write your temperature in {unit}");
    let temp = io::input_float();

    println!("In what unit do you want to convert it? (K, C, F)");
    let unit_new = io::input_char().to_ascii_uppercase();

    validate_unit(unit_new);

    let temp_new = convert(temp, unit, unit_new);
    println!("{temp} {unit} is also {temp_new} {unit_new}");
}

fn convert(temp: f32, unit: char, unit_new: char) -> f32 {
    match (unit, unit_new) {
        ('K','C') => temp::kelvin_to_celsius(temp),
        ('K','F') => temp::kelvin_to_farenheit(temp),
        ('C','K') => temp::celsius_to_kelvin(temp),
        ('C','F') => temp::celsius_to_farenheit(temp),
        ('F','K') => temp::farenheit_to_kelvin(temp),
        ('F','C') => temp::farenheit_to_celsius(temp),
        _ => return temp
    }
}

fn validate_unit(unit: char) {
    match unit {
        'K' | 'C' | 'F' => (),
        _ => panic!("Error: Invalid unit")
    }
}