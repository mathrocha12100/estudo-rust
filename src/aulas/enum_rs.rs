
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

enum Aula {
    Aula1 = 1,
    Aula2 = 2,
    Aula3 = 3,
    Aula4 = 4,
}

fn aula_01_enums() {
    let player: Direction = Direction::Right;

    match player {
        Direction::Up => println!("O jogador foi para cima"),
        Direction::Down => println!("O jogador foi para baixo"),
        Direction::Left => println!("O jogador foi para a esquerda"),
        Direction::Right => println!("O jogador foi para a direita"),
    } 

    let player_male: Gender = Gender::Male;
    let player_female: Gender = Gender::Female;

    println!("{:?}", player_male);
    println!("{:?}", player_female);
}

enum CarType {
    Fiat,
    Ford,
    Renault,
}

fn car_nacionality(car: CarType) {
    match car {
        CarType::Fiat => println!("Italiano"),
        CarType::Ford => println!("Americano"),
        CarType::Renault => println!("Françês"),
    }
}

fn aula_02_enums() {
    car_nacionality(CarType::Fiat);
    car_nacionality(CarType::Ford);
    car_nacionality(CarType::Renault);
}

enum Payment {
    Money,
    CreditCard,
    Paypal,
}

fn aula_03_enums() {
    let payment: Payment = Payment::CreditCard;

    match payment {
        Payment::CreditCard => println!("credit card"),
        Payment::Money => println!("dinheiro"),
        _ => {} // ignore other enums
    }
}

enum PaymentType {
    Money(f32),
    CreditCard(bool, f32),
    Paypal(bool, f32),
}

fn aula_04_enums() {
    let payment: PaymentType = PaymentType::CreditCard(false, 100f32);

    match payment {
        PaymentType::Money(f) => println!("A pessoa pgaou em dinheiro {}", f), 
        PaymentType::CreditCard(true, amount) => println!("Deu bomA pessoa pgaou em dinheiro {}",amount), 
        PaymentType::CreditCard(false, _) => println!("Deu ruim nao passou"), 
        _ => {} // ignore other enums
    }

}

pub fn enum_rs() {
    let aula: Aula = Aula::Aula4;

    match aula {
        Aula::Aula1 => aula_01_enums(),
        Aula::Aula2 => aula_02_enums(),
        Aula::Aula3 => aula_03_enums(),
        Aula::Aula4 => aula_04_enums(),
    }
}