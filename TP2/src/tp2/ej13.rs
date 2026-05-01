pub fn ordenar_nombres<const N:usize>(arr : &mut [String;N]){
    arr.sort();
}

#[cfg(test)]
mod testing_ejercicio13{
    use crate::tp2::ej13;

    #[test]
    fn caso_especial(){
        let mut arr : [String;0] = [String::from("");0]; 
        ej13::ordenar_nombres(&mut arr);
        assert_eq!(arr,[String::from("");0]);
    }

    #[test]
    fn arreglo_nombres(){
        let mut arr= [String::from("Savedra"),String::from("Pedro"),String::from("Martina")]; 
        ej13::ordenar_nombres(&mut arr);
        assert_eq!(arr,[String::from("Martina"),String::from("Pedro"),String::from("Savedra")]);
        arr[0] = String::from("Alan");
        assert_eq!(arr,[String::from("Alan"),String::from("Pedro"),String::from("Savedra")]);
    }

    #[test]
    fn arreglo_usuarios(){
        let mut arr= [String::from("."),String::from(""),String::from("Martu234"),String::from("._.Messi10"),"".to_string()];
        ej13::ordenar_nombres(&mut arr);
        assert_eq!(arr,[String::from(""),"".to_string(),String::from("."),String::from("._.Messi10"),String::from("Martu234")]);
        
        arr[0] = String::from("ElMasCapito7u7");
        arr[1] = String::from("RIVERELMASGRANDE!!");
        assert_eq!(arr,[String::from("ElMasCapito7u7"),String::from("RIVERELMASGRANDE!!"),String::from("."),String::from("._.Messi10"),String::from("Martu234")]);
    
        ej13::ordenar_nombres(&mut arr);
        assert_eq!(arr,[String::from("."),String::from("._.Messi10"),String::from("ElMasCapito7u7"),String::from("Martu234"),String::from("RIVERELMASGRANDE!!")]);
    }

}