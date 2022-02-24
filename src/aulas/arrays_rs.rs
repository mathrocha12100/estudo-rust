use sysinfo::NetworksExt;

pub fn arrays_rs() {
    let numbers = [1, 2, 3, 4];
    let ar = [2;1000];
    println!("{}", numbers[0]);

    for number in numbers.iter() {
        println!("{}", number)
    } 
}