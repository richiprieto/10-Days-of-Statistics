use std::io;

fn main() {
    // Lectura de parametros de ingreso
    let mut max = String::new();
    io::stdin().read_line(&mut max).expect("Error");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error");

    let mut mean = String::new();
    io::stdin().read_line(&mut mean).expect("Error");

    let mut sd = String::new();
    io::stdin().read_line(&mut sd).expect("Error");

    // Convertimos n en f32
    let max: f32 = max.trim().parse().expect("Error");
    let n: f32 = n.trim().parse().expect("Error");
    let mean: f32 = mean.trim().parse().expect("Error");
    let sd: f32 = sd.trim().parse().expect("Error");

    // Respuesta
    println!("{:.4}", clt(max, n * mean, n.sqrt() * sd));
}

fn clt (max: f32, mean: f32, sd: f32) -> f32 {
    let x = (max - mean) / (sd * 2.0_f32.sqrt());
    let resp1 = 0.5 * (1.0 + erf(x));
    resp1
}

fn erf (x: f32) -> f32{
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;

    let mut signo = 1.0;

    if x < 0.0 {
        signo = -1.0;
    }

    let x = x.abs();

    let t = 1.0/(1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2)* t + a1) * t * (-x * x).exp();

    signo * y
}
