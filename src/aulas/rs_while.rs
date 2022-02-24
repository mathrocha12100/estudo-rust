use std::io;

fn convert_to_int(string_number: &String) -> i32 {
    let number =  string_number.trim().parse::<i32>().unwrap();
    return number;
}

pub fn rs_while() { 
    let mut soma = 0;
    println!("Informe o primeiro numero: ");
    let mut str_number = String::new();
    io::stdin().read_line(&mut str_number).expect("Erro ao ler number 1");
    let mut int_number = convert_to_int(&str_number);

    while int_number != 0 {
        let r = int_number % 10;
        soma = soma + r;
        int_number = int_number / 10;
    }

    println!("O valor da soma dos digitos é {}", soma);

}