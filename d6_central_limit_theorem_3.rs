use std::io;

fn main() {
    // Lectura de parametros de ingreso
    let mut sample = String::new();
    io::stdin().read_line(&mut sample).expect("Error");

    let mut mean = String::new();
    io::stdin().read_line(&mut mean).expect("Error");

    let mut sd = String::new();
    io::stdin().read_line(&mut sd).expect("Error");

    let mut per_dist = String::new();
    io::stdin().read_line(&mut per_dist).expect("Error");
    
    let mut z = String::new();
    io::stdin().read_line(&mut z).expect("Error");

    // Convertimos las variables en f32
    let sample: f32 = sample.trim().parse().expect("Error");
    let mean: f32 = mean.trim().parse().expect("Error");
    let sd: f32 = sd.trim().parse().expect("Error");
    let per_dist: f32 = per_dist.trim().parse().expect("Error");
    let z: f32 = z.trim().parse().expect("Error");

    // Muestra de la desviacion estandar
    let sd_sample = sd / sample.sqrt();

    // Respuestas
    println!("{:.2}", mean - sd_sample * z);
    println!("{:.2}", mean + sd_sample * z);
}
