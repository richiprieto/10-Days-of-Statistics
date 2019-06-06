use std::io;

fn main(){
    /*Leemos los dos ingresos de datos*/
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error");

    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect("Error");

    let mut freq = String::new();
    io::stdin().read_line(&mut freq).expect("Error");
    
    //Convertimos en un vector u32
    let mut valor: Vec<i32> = valor
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    //Convertimos en un vector u32
    let mut freq: Vec<i32> = freq
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Creamos el vector en funcion de la frecuencia de los valores
    let mut vector = Vec::new();
    for (&v, &f) in valor.iter().zip(freq.iter()){
        for x in 0..f{
            vector.push(v);
        }
    }

    vector.sort();
    let mitad = vector.len() / 2;
    let residuo = vector.len() % 2;
    
    if residuo == 0 { 
        // Q1 - Q2
        println!("{:.1}", mediana(&mut vector[mitad..]) - mediana(&mut vector[..=mitad-1]) as f32 );
    } else { 
        // Q3 - Q2
        println!("{:.1}", mediana(&mut vector[mitad+1..]) - mediana(&mut vector[..=mitad-1]) as f32 );
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
