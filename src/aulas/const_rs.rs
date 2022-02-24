
const NUMERO_PI: f32 = 3.14159;

fn comprimento_circuferencia(r: f32) -> f32 {
    let c = 2f32 * r * NUMERO_PI;

    return c;
}

pub fn const_rs() {
    println!("comprimento {}", comprimento_circuferencia(2.32));
}

