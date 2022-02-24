struct User {
    username: String,
    email: String,
    active: bool,
    gender: String,
}

fn print_user_name(user: &User) {
    println!("O nome do usuário é {}", user.username);
}

pub fn structs_rs() {
    let mut people = User { username: String::from("João Pessoa"), email: String::from("joao@gmail.com"), active: true, gender: String::from("Male") };

    print_user_name(&mut people);

    people.username = String::from("Cleiton");

    print_user_name(&mut people);

}  