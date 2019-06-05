use std::io;

fn main(){
    let mut n = String::new(); 
    io::stdin().read_line(&mut n).expect("Fallo");

    let mut values = String::new(); 
    io::stdin().read_line(&mut values).expect("Fallo");


    let mut pesos = String::new(); 
    io::stdin().read_line(&mut pesos).expect("Fallo");

    let values: Vec<f32> = values
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let pesos: Vec<f32> = pesos
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut nomin: f32 = 0.0;
    let mut denom: f32 = 0.0;

    for (&v, &w) in values.iter().zip(pesos.iter()){
        nomin += v * w;
        denom += w;
    }
    println!("{:.1}", nomin/denom);  
}
