pub fn longitud_de_cadenas<const N:usize>(arr : &[String;N])->[u32;N]{
    let mut res = [0;N];
    for i in 0..N{
        res[i] = arr[i].len() as u32;
    }
    return res;
}

#[cfg(test)]
mod testing_ejercicio6{
    use crate::tp2::ej6;

    #[test]
    fn casos_especiales(){
        let arr : [String;0] = [String::from("");0]; 
        assert_eq!(ej6::longitud_de_cadenas(&arr),[0;0]);
        let arr = [String::from("")]; 
        assert_eq!(ej6::longitud_de_cadenas(&arr),[0]);
    }

    #[test]
    fn array_apalabrado(){
        let mut arr = [String::from(""),String::from("Hola"),String::from("Adios"),String::from("Rust")];
        assert_eq!(ej6::longitud_de_cadenas(&arr),[0,4,5,4]);
        arr[0] = "Nuevo".to_string();
        assert_eq!(ej6::longitud_de_cadenas(&arr),[5,4,5,4]);
    }
}