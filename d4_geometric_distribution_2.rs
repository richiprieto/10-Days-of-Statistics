use std::io;

fn main() {
    let mut vector = String::new();
    io::stdin().read_line(&mut vector).expect("Error");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error");

    // Convertimos n en u32
    let n: u32 = n.trim().parse().expect("Error");

    //Convertimos en un vector u32
    let mut vector: Vec<f32> = vector
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut suma = 0.0;
    let p = vector[0] / vector[1];
    
    for i in 1..6 {
        suma += (1.0 - p).powf((5 - i) as f32) * p;
    }
    
    println!{"{:.3}", suma};
}
