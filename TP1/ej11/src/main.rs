use std::io::stdin;

fn main() {
    let mut arr: [String; 5] = [
        String::new(), 
        String::new(), 
        String::new(), 
        String::new(), 
        String::new()
    ];

    for i in 0..arr.len() {
        let mut read_in = String::new();
        stdin().read_line(&mut read_in).expect("Error de lectura");
        let cad2 = read_in.trim().to_string();
        arr[i] = cad2;
        //En el caso de que no haya ocurrido una excepcion en la lectura , se imprimira el siguiente mensaje 
        println!("Se guardo la cadena en el arreglo");
    }

    //println!("{:?}",arr);
}
