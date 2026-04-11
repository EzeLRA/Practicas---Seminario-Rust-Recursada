use std::io::stdin;

fn main() {
    const CAD : &str = "Una palabra";
    let mut cant : u32 = 0;

    let mut read_in = String::new();
    
    stdin().read_line(&mut read_in).expect("Error de lectura");

    let caract = read_in.trim().chars().next().expect("Error de extraccion de caracter");

    for c in CAD.chars(){
        if c == caract {
            cant += 1;
        }
    }

    println!("{}",cant);

}
