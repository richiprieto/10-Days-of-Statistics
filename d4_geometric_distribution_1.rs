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
    
    // Distribucion geometrica
    let p = vector[0] / vector[1];
    let result = (1.0 - p).powf((n - 1) as f32) * p;

    println!{"{:.3}", result};
}
