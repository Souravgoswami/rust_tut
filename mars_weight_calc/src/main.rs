use std::io ;
use std::io::Write ; // << --- flush() into scope

fn main() {
    // let mars_weight = calculate_weight_on_mars(100.0) ;
    // let mars_weight: f64 = calculate_weight_on_mars(100.0).into() ;

    // let mut mars_weight_1000 = calculate_weight_on_mars(100.0) ;
    // mars_weight_1000 *= 1000.0 ;

    let mut input = String::new() ;

    print!("Enter your weight on Earth (KG): ");
    io::stdout().flush().unwrap() ;

    io::stdin().read_line(&mut input).unwrap() ;

    let input_float: f32 = input.trim().parse().unwrap() ;
    let weight: f32 = calculate_weight_on_mars(input_float) ;

    println!("Weight: {} KG", weight) ;
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
    weight_on_earth / 9.81 * 3.711
}
