use std::io;

fn main() {
    let mut vector = String::new();
    io::stdin().read_line(&mut vector).expect("Error");

    //Convertimos en un vector u32
    let mut vector: Vec<f32> = vector
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let res1 = 160.0 + 40.0 * (vector[0] + vector[0].powf(2.0));
    let res2 = 128.0 + 40.0 * (vector[1] + vector[1].powf(2.0));

    println!("{:.3}", res1);
    println!("{:.3}", res2);
}
