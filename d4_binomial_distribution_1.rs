use std::io;

fn main() {
    let mut vector = String::new();
    io::stdin().read_line(&mut vector).expect("Error");

    //Convertimos en un vector u32
    let mut vector: Vec<f32> = vector
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let odds = vector[0] / vector[1];
    let mut suma = 0.0;
    for i in 3..7{
        suma += binomial(i as f32, 6.0, odds / (1.0 + odds));
    }
    println!("{:.3}", suma);
}

fn factorial(n: f32) -> f32{
    if n == 0.0 {
        1.0
    }
    else {
        n * factorial(n-1.0)
    }
}

fn combinacion(n: f32, r: f32) -> f32{
    factorial(n) / (factorial(r) * factorial(n-r))
}

fn binomial(r: f32, n: f32, p: f32) -> f32{
    combinacion(n, r) * p.powf(r) * (1.0-p).powf((n-r))
}
