use std::io::stdin;

fn main() {
    let cad1 = "Hola soy ".to_string();
    
    let mut cad2 = String::new();
    
    stdin().read_line(&mut cad2).expect("Error");

    let res : String = cad1 + cad2.trim();

    println!("{}", res.to_uppercase());
}
