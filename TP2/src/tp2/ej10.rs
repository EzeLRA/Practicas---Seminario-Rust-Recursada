pub fn cantidad_de_cadenas_mayor_a<const N:usize>(arr : &[String;N],limite : u32)->u32{
    let mut cant = 0;
    
    for i in 0..N{
        if arr[i].len() as u32 > limite {
            cant += 1;
        }
    }

    return cant;
}

#[cfg(test)]
mod testing_ejercicio10{
    use crate::tp2::ej10;

    #[test]
    fn casos_especiales(){
        let arr : [String;0] = [String::from("");0]; 
        assert_eq!(ej10::cantidad_de_cadenas_mayor_a(&arr, 0),0);
        let arr = [String::from("")]; 
        assert_eq!(ej10::cantidad_de_cadenas_mayor_a(&arr, 0),0);
        let arr = [String::from("unacosa")]; 
        assert_eq!(ej10::cantidad_de_cadenas_mayor_a(&arr, 0),1);
    }

    #[test]
    fn array_apalabrado(){
        let mut arr = [String::from(""),String::from("Hola"),String::from("Adios"),String::from("Rust")];
        assert_eq!(ej10::cantidad_de_cadenas_mayor_a(&arr, 3),3);
        arr[0] = "Nuevo".to_string();
        assert_eq!(ej10::cantidad_de_cadenas_mayor_a(&arr, 3),4);
    }
}