pub fn vector_rs() {
    let mut vetores = vec![1, 2, 3, 4];
    
    vetores.push(6);
    println!("{:?}", vetores);

    vetores.remove(1);

    println!("{:?}", vetores);

    for i in vetores.iter() {
        println!("{:?}", i);
    }

}