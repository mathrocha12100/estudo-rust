use std::io;

fn convert_to_int(string_number: &String) -> i32 {
    let number =  string_number.trim().parse::<i32>().unwrap();
    return number;
}

pub fn factorial_rs() {
    let mut fac_str = String::new();
    println!("Digite o fatorial: ");
    io::stdin().read_line(&mut fac_str).expect("Erro ao ler fac");
    
    let mut fac = 1;
    let mut fac_int = convert_to_int(&fac_str);

    while fac_int > 1 {
        fac = fac * fac_int;
        fac_int -= 1;
    }

    println!("Resultado: {}", fac);
}

