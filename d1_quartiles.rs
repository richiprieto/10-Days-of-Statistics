use std::io;

fn main(){
    /*Leemos los dos ingresos de datos*/
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error");

    let mut vector = String::new();
    io::stdin().read_line(&mut vector).expect("Error");
    
    //Convertimos en un vector u32
    let mut vector: Vec<i32> = vector
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    vector.sort();
    let mitad = vector.len() / 2;
    let residuo = vector.len() % 2;
    
    if residuo == 0 { 
        // Q1
        println!("{}", mediana(&mut vector[..=mitad-1]));
        // Q2
        println!("{}", mediana(&mut vector));
        // Q3
        println!("{}", mediana(&mut vector[mitad..]));
    } else { 
        // Q1
        println!("{}", mediana(&mut vector[..=mitad-1]));
        // Q2
        println!("{}", mediana(&mut vector));
        // Q3
        println!("{}", mediana(&mut vector[mitad+1..]));
    }
}

fn media (vector: &[i32]) -> f32 {
    let suma: i32 = vector.iter().sum();
    suma as f32/vector.len() as f32
}

fn mediana (vector: &mut [i32]) -> f32 {
    let mitad = vector.len() / 2;
    let residuo = vector.len() % 2;
    if residuo == 0{
        media(&vector[(mitad - 1)..(mitad + 1)])
    } else {
        vector[mitad] as f32
    }
}
