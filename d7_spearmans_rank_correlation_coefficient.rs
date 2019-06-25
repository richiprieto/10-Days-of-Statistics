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
    let vector_x: Vec<f32> = vector_x
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Convertimos vector_y a f32
    let vector_y: Vec<f32> = vector_y
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Obtenemos las posiciones de los indices ordenados
    // en vectores
    let rx = obtencion_indices(&vector_x);
    let ry = obtencion_indices(&vector_y);

    // Obtenemos sumatoria de d^2
    let mut d = 0.0;
    for i in 0 as usize..n as usize {
        d += (rx[i] - ry[i]).powf(2.0)
    }

    // Obtenemos el valor del coeficiente
    let respuesta = 1.0 - (6.0 * d) / (n * (n * n - 1.0));
    println!("{:.3}", respuesta);
}

fn obtencion_indices(vector: &Vec<f32>) -> Vec<f32>{
    let mut result: Vec<f32> = Vec::new();
    let mut vec = vector.clone();

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for i in vector {
        let index = vec.iter().position(|&r| r == *i).unwrap();
        result.push(index as f32);
    }
    result
}
