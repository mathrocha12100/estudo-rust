use std::io;

fn convert_to_int(string_number: &String) -> i32 {
    let number =  string_number.trim().parse::<i32>().unwrap();
    return number;
}

fn factorial() {
    let mut fac_str = String::new();

    io::stdin().read_line(&mut fac_str).expect("Erro ao ler fac_str");

    let fac_int = convert_to_int(&fac_str);
    let mut sum = 1;

    for i in 1..fac_int {

        println!("a: {} ", fac_int - i);
    }
}

pub fn rs_for() {
    factorial();
}