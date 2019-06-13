use std::io;
use std::f32::consts::E;

fn main() {
    let mut lambda = String::new();
    io::stdin().read_line(&mut lambda).expect("Error");

    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("Error");

    // Convertimos n en f32
    let lambda: f32 = lambda.trim().parse().expect("Error");
    let k: f32 = k.trim().parse().expect("Error");

    // Calculamos Poisson
    let result = (lambda.powf(k) * E.powf(-lambda)) / factorial(k);
    println!("{:.3}", result);

}

fn factorial(n: f32) -> f32{
    if n == 0.0 {
        1.0
    }
    else {
        n * factorial(n-1.0)
    }
}
