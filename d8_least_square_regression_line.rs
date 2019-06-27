fn main() {
    let x = vec![95, 85, 80, 70, 60];
    let y = vec![85, 95, 70, 65, 70];
    let n = x.len();
    
    let x_sum: i32 = sumatoria(&x);
    let y_sum: i32 = sumatoria(&y);
    
    let x_prom: i32 = promedio(&x_sum, &(n as i32));
    let y_prom: i32 = promedio(&y_sum, &(n as i32));
    
    let x_2: i32 = x_2_sum(&x);
    let x_y: i32 = x_y(&x, &y);
    
    // Coeficientes de la recta
    let (a, b) = coef_a_b(&(n as i32), &x_y, &x_sum, &y_sum, &x_2, &y_prom, &x_prom);
    
    // Obtenemos Y en funcion de X
    let x_referencia = 80.0;
    println!("{:.3}", a + b * x_referencia);
}

fn coef_a_b (n: &i32, x_y: &i32, x_sum: &i32, y_sum: &i32, x_2: &i32, y_prom: &i32, x_prom: &i32) -> (f32, f32){
    let b: f32 = (*n as i32 * x_y - x_sum * y_sum) as f32 / (*n as i32 * x_2 - x_sum.pow(2)) as f32;
    let a: f32 = *y_prom as f32 - b * *x_prom as f32;
    (a,b)
}

fn x_y (vector_x: &Vec<i32>, vector_y: &Vec<i32>) -> i32 {
    let mut sumatoria: i32 = 0;
    for (x, y) in vector_x.iter().zip(vector_y) {
        sumatoria += x * y;
    }
    sumatoria
}

fn x_2_sum (vector: &Vec<i32>) -> i32 {

    let mut sumatoria: i32 = 0;
    for i in vector {
        sumatoria += i.pow(2);
    }
    sumatoria
}

fn sumatoria(vector: &Vec<i32>) -> i32 {
    vector.iter().sum()
}

fn promedio(sumatoria: &i32, n: &i32) -> i32 {
    sumatoria / n
}
