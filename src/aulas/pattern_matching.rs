pub fn pattern_matching() {
    let number = 2;

    match number {
        1 => println!("1"),
        2 | 3 => println!("2 ou 3"),
        2..=10 => println!("entre 2 e 10"),
        _ => println!("no")
    }
}