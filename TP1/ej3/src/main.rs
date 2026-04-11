use std::io::stdin;

fn main() {
    let bool1 : bool = false;
    let mut read_in = String::new();
    stdin().read_line(&mut read_in).expect("Error de lectura");
    
    let bool2 = read_in.trim().parse().expect("Error de parseo");
    
    println!("AND: {}", bool1 && bool2);
    println!("OR: {}", bool1 || bool2);
}
