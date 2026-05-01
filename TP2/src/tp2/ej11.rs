pub fn multiplicar_valores<const N:usize>(arr : &mut [i32;N], factor : i32){
    for i in 0..N{
        arr[i] *= factor;
    }
}

#[cfg(test)]
mod testing_ejercicio11{
    use crate::tp2::ej11;

    #[test]
    fn caso_especial(){
        let mut arr1 = [0;0];
        ej11::multiplicar_valores(&mut arr1, 2);
        assert_eq!(arr1,[0;0]);
    }

    #[test]
    fn factor_cero(){
        let mut arr1 = [1,2,3,4,5];
        ej11::multiplicar_valores(&mut arr1, 0);
        assert_eq!(arr1,[0;5]);
    }


    #[test]
    fn factor_positivo(){
        let mut arr1 = [1,-2,3,-4,5];
        ej11::multiplicar_valores(&mut arr1, 2);
        assert_eq!(arr1,[2,-4,6,-8,10]);
    }

    #[test]
    fn factor_negativo(){
        let mut arr1 = [1,-2,3,-4,5];
        ej11::multiplicar_valores(&mut arr1, -2);
        assert_eq!(arr1,[-2,4,-6,8,-10]);
    }
}