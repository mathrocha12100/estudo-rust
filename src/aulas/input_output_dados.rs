use std::io;

fn convert_to_int(string_number: &String) -> i32 {
    let number =  string_number.trim().parse::<i32>().unwrap();
    return number;
}

pub fn exemplo_entrada_saida_dados() {
    println!("Informe o primeiro numero: ");
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Erro ao ler number 1");
    let mut number2 = String::new();
    println!("Informe o segundo numero: ");
    io::stdin().read_line(&mut number2).expect("Erro ao ler number 2");
    println!("resultado: ");

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!("{} > {}", number1, number2);
        return;
    }
    println!("{} <= {}", number1, number2);
}