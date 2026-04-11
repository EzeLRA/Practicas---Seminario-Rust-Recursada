use std::io::stdin;

fn main() {
    //Variable
    let x : f32 = 0.0;

    //Lector
    let mut ent = String::new();
    //Lectura en consola
    stdin().read_line(&mut ent).expect("Error");
    
    //Conversion de la entrada
    let num:f32 = ent.trim().parse().expect("No es un flotante");
    
    //Operaciones
    println!("Suma = {}",(x + num));

    println!("Resta = {}",(x - num));

    println!("Multiplicacion = {}",(x * num));

    println!("Division = {}",(x / num));
}
