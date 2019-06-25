use std::io;
fn main(){
    /*Leemos los dos ingresos de datos*/
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error");

    let mut vector_x = String::new();
    io::stdin().read_line(&mut vector_x).expect("Error");

    let mut vector_y = String::new();
    io::stdin().read_line(&mut vector_y).expect("Error");
    // Convertimos ingreso de n en f32
    let n: f32 = n.trim().parse().expect("Error");

    // Convertimos vector_x a f32
    let mut vector_x: Vec<f32> = vector_x
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Convertimos vector_y a f32
    let mut vector_y: Vec<f32> = vector_y
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Obtenemos media y desviacion estandar de "x"
    let media_x = media(&vector_x);
    let mean_square_x = mean_square(&vector_x, &media_x);
    let std_dev_x = std_dev(&mean_square_x, &n);

    // Obtenemos media y desviacion estandar de "y"
    let media_y = media(&vector_y);
    let mean_square_y = mean_square(&vector_y, &media_y);
    let std_dev_y = std_dev(&mean_square_y, &n);

    // Calculo de Numerador de Pearson Correlation Coefficient
    let mut numerador = 0.0;
    for (x, y) in vector_x.iter().zip(vector_y) {
        numerador += (x - media_x) * (y - media_y);
    }
    
    // Calculo de Pearson Correlation Coefficient
    let respuesta = numerador / (n * std_dev_x * std_dev_y);

    // Presentamos la respuesta
    println!("{:.3}", respuesta);
}

fn std_dev (mean_square: &f32, n: &f32) -> f32{
    (mean_square / n).sqrt()
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
