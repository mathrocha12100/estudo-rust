
struct People {
    name: String,
    age: i32,
}

trait Voice {
    fn speak(&self);
    fn have_voice(&self) -> bool;
}

impl Voice for People {
    fn speak(&self) {
        println!("Olá meu nome é {}", self.name);
    }
    fn have_voice(&self) -> bool {
        if self.age > 1 {
            return true;
        }
        return false;
    }
   
}

pub fn traits() {
    let people = People{ name: String::from("Matheus"), age: 20 };

    if people.have_voice() {
        people.speak();
    } 
}