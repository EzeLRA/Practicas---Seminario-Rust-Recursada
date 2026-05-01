pub fn incrementar(numero : &mut f32){
    *numero += 1.0;
}

#[cfg(test)]
mod testing_ejercicio14{
    use crate::tp2::ej14;

    #[test]
    fn numero_negativo(){
        let mut num = -4.51;
        ej14::incrementar(&mut num);
        assert_eq!(num,-3.5100002);
        num = -1.0;
        ej14::incrementar(&mut num);
        assert_eq!(num,0.0);
    }

    #[test]
    fn numero_positivo(){
        let mut num = 0.0;
        ej14::incrementar(&mut num);
        assert_eq!(num,1.0);
        num = 2.32;
        ej14::incrementar(&mut num);
        assert_eq!(num,3.32);
    }
}