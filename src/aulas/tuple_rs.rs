fn tple() {
    let tuple = (12, "valores", 3.14, (1, 2, 3));

    let ( a, b , c, d ) = tuple;

    // println!("{}\n ",tuple.0);
    // println!("{}\n",tuple.1);
    // println!("{}\n ",tuple.2);
    // println!("{}\n ",tuple.3.1);

    println!("{}\n ",a);
    println!("{}\n",b);
    println!("{}\n ",c);
    println!("{:?}\n ",d); // common :?
    println!("{:#?}\n ",d); // pretty-print :#?
    println!("tuple {:#?}\n ",tuple.3.1); // pretty-print :#?
}

struct User (String, String, bool, String);

pub fn tuple_rs() {
   let mut people: User = User(String::from("Matheus"), String::from("adwa0"), false, String::from("dswadwa"));
   println!("O nome do usuário é {}", people.0);

   people.0 = String::from("Cleiton");
   println!("O nome do usuário é {}", people.0);

}