
pub fn celsius_to_farenheit(temp: f32) -> f32 {
    return temp * 9.0/5.0 + 32.0;
}

pub fn celsius_to_kelvin(temp: f32) -> f32 {
    return temp + 273.15;
}

pub fn kelvin_to_celsius(temp: f32) -> f32 {
    return temp - 273.15;
}

pub fn farenheit_to_celsius(temp: f32) -> f32 {
    return (temp - 32.0) * 5.0/9.0; 
}

pub fn farenheit_to_kelvin(temp: f32) -> f32 {
    return ((temp - 32.0) * 5.0/9.0) + 273.15; 
}

pub fn kelvin_to_farenheit(temp: f32) -> f32 {
    return (temp - 273.15) * 9.0/5.0 + 32.0;
}
