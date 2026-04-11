use std::io::stdin;

fn main() {
    let a : u8 = 10;

    let mut read_in = String::new();
    
    stdin().read_line(&mut read_in).expect("Error");

    let b : u8 = read_in.trim().parse().expect("Error de parseo");

    println!("{}", u8::pow( a+b , 2) );

}
