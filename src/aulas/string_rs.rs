pub fn string_rs() {
    let mut myString: String = String::from("Ola João");

    println!("o tamanho dessa string é {}", myString.len());
    println!("A minha string está vazia ? {}", myString.is_empty());

    for token in myString.split_whitespace() {
        println!("{}", token);
    }

    println!("a string contem 'Ola' ? {}", myString.contains("Ola"));

    myString.push_str(" jejo");

    println!("myString {}", myString)
}