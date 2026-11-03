

fn main() {
   let temperature_celcius:f64 = 10.0;
    println!("{}", celcius_to_fahrenheit(temperature_celcius));
   let temperature_fahrenheit:f64 = 60.0; 
    println!("{}", fahrenheit_to_celcius(temperature_fahrenheit));
}
fn celcius_to_fahrenheit(c:f64) -> f64 {
   let temp:f64 = (c * 9.0/5.0) + 32.0;
    return temp;
}

fn fahrenheit_to_celcius(f:f64) -> f64 {
    let temp:f64 = (f - 32.0) * (5.0/9.0);
    return temp;
}