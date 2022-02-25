use std::collections::HashMap;

pub fn hash_map() {
    let mut hash_map = HashMap::new();

    hash_map.insert("Matematica", 10);
    hash_map.insert("PT", 7);
    hash_map.insert("Inglês", 9);
    hash_map.insert("Informatica", 9);

    println!("{:?}",hash_map);
    println!("Quantas materias o aluno cursou? {}", hash_map.len());

    match hash_map.get("Informatica") {
        Some(k ) => println!("valor {}", k),
        None => println!("alu nao cursou informatica")
    }

    hash_map.remove("PT");

    println!("O aluno estuda Matematica ? {}", hash_map.contains_key("Matematica"))

}