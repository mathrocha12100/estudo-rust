struct User {
    username: String,
    email: String,
    isActive: bool,
    gender: String,
}

impl User {
    fn sayUserName(&self) {
        println!("o nome do usuário é {}", self.username);
    }

    fn userIsActive(&self) {
        println!("Está ativo ? {}", self.isActive);
    }
}



pub fn keyword() {
    let pessoa: User = User{ username: String::from("Matheus Rocha"), email: String::from("matheus@gmail.com"), isActive: true, gender: String::from("Masculino") };

    pessoa.sayUserName();
    pessoa.userIsActive();
}