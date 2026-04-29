pub fn cantidad_en_rango<const N:usize>(arr : [i32;N], inferior : i32 , superior : i32)->u32{
    let mut cant = 0;
    
    for num in arr{
        if (num >= inferior)&(num <= superior) {
            cant += 1;
        }
    }

    return cant;
}

#[cfg(test)]
mod testing_ejercicio9{
    use crate::tp2::ej9;

    #[test]
    fn caso_especial(){
        let arr = [0;0];
        assert_eq!(ej9::cantidad_en_rango(arr, 0, 0),0);
        assert_eq!(ej9::cantidad_en_rango(arr, -5, 5),0);
    }

    #[test]
    fn rango_negativo(){
        let arr = [0,2,-7,-5,-10,7,-23];
        assert_eq!(ej9::cantidad_en_rango(arr, -10, -5),3);
    }

    #[test]
    fn rango_positivo(){
        let arr = [0,5,8,-1,-2,10];
        assert_eq!(ej9::cantidad_en_rango(arr, 0, 8),3);
    }

    #[test]
    fn rango_mixto(){
        let arr = [0,5,8,-1,-2,-10];
        assert_eq!(ej9::cantidad_en_rango(arr, -10, 8),6);
    }

}