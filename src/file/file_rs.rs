use std::fs::File;
use std::io::prelude::*;

fn read_file(fle: &mut File, content: &mut String) {
    fle.read_to_string(content).expect("Não foi possivel alocar na variavel content");

    println!("conteudo {}", content)
}

fn write_file(file: &mut File) {
    file.write_all(b"Arquivo teste criado!").expect("nao conseguiu escrever no arq");
}

pub fn file_rs() {
    let mut file = File::create("teste.txt").expect("Não conseguiu criar um arquivo");

    write_file(&mut file);
    let mut content = String::new();

    read_file(&mut file, &mut content);



}