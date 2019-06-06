use std::io;

fn main(){
    /*Leemos los dos ingresos de datos*/
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error");

    let mut vector = String::new();
    io::stdin().read_line(&mut vector).expect("Error");

    //Convertimos en un vector f32
    let mut vector: Vec<f32> = vector
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let media = media(&vector);
    let mean_square = mean_square(&vector, &media);
    //Desvia. Estandar
    println!("{:.1}", (mean_square / vector.len() as f32).sqrt()); 
}

fn media (vector: &[f32]) -> f32 {
    let suma: f32 = vector.iter().sum();
    suma / vector.len() as f32
}

fn mean_square (vector: &[f32], media: &f32) -> f32 {
    let mut resultado = 0.0;
    for value in vector {
        resultado += (value - media).powf(2.0); 
    }
    resultado
}
