static mut STATIC_VAR: i32 = 15;

pub fn shadowing_rs() {
    let a: i32 = 10;

    {
        println!("O Valor de a é {}", a);
        let a: f32  = 15.3;
        println!("O Valor de a é {}", a);

        unsafe {
            println!("O Valor de STATIC_VAR é {}", STATIC_VAR);
        }
    }

    println!("O Valor de a é {}", a);
    
    unsafe {
        println!("O Valor de STATIC_VAR é {}", STATIC_VAR);
    }

}