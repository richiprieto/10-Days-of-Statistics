fn main() {

    // Lectura de parametros de ingreso
    let mut mean_sd = String::new();
    io::stdin().read_line(&mut mean_sd).expect("Error");

    let mut prob1 = String::new();
    io::stdin().read_line(&mut prob1).expect("Error");

    let mut prob2 = String::new();
    io::stdin().read_line(&mut prob2).expect("Error");

    // Convertimos n en f32
    let prob1: f32 = prob1.trim().parse().expect("Error");
    
    // Convertimos en vector f32
    let mut mean_sd: Vec<f32> = mean_sd
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut prob2: Vec<f32> = prob2
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();


    // Pregunta 1
    let x = (prob1 - mean_sd[0]) / (mean_sd[1] * 2.0_f32.sqrt());
    let resp1 = 0.5 * (1.0 + erf(x));
    println!("{:.3}", resp1);

    // Pregunta 2
    let x1 = (prob2[0] - mean_sd[0]) / (mean_sd[1] * 2.0_f32.sqrt());
    let x2 = (prob2[1] - mean_sd[0]) / (mean_sd[1] * 2.0_f32.sqrt());
    let resp2_1 = 0.5 * (1.0 + erf(x1));
    let resp2_2 = 0.5 * (1.0 + erf(x2));
    println!("{:.3}", resp2_2 - resp2_1);

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

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2)* t + a1) * t * (-x * x).exp();

    signo * y
}
