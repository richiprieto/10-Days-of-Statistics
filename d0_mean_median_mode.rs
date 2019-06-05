// Enter your code here 
use std::io;
use std::collections::BTreeMap;

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

    // Obtenemos la media
    println!("{:.1}", media(&vector));
    // obtenemos la mediana
    println!("{:.1}", mediana(&mut vector));
    // obtenemos la moda
    println!("{}", moda(&vector));

}

fn media (vector: &[i32]) -> f32 {
    let suma: i32 = vector.iter().sum();
    suma as f32/vector.len() as f32
}

fn mediana (vector: &mut [i32]) -> f32 {
    vector.sort();
    let mitad = vector.len() / 2;
    let residuo = vector.len() % 2;
    if residuo == 0{
        media(&vector[(mitad - 1)..(mitad + 1)])
    } else {
        vector[mitad] as f32
    }
}

fn moda(vector: &[i32]) -> i32 {
    let mut repeticiones = BTreeMap::new();
    for &valor in vector {
        *repeticiones.entry(valor).or_insert(0) += 1;
    }
    repeticiones
        .into_iter().rev()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("No se puede computar moda de vector de ceros")
}
