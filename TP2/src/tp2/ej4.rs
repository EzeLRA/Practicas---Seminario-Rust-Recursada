pub fn cantidad_impares<const N:usize>(arr : [i32;N])->u32{
    let mut cant : u32  = 0;
    for num in arr {
        if num % 2 != 0 {
            cant = cant + 1;
        }
    }
    cant
}

#[cfg(test)]
mod testing_ejercicio4{
    use crate::tp2::ej4;

    #[test]
    fn casos_especiales(){
        let arr : [i32;0] = [0;0];
        assert_eq!(ej4::cantidad_impares(arr),0);
        let arr : [i32;5] = [2;5];
        assert_eq!(ej4::cantidad_impares(arr),0);
    }

    #[test]
    fn array_mixto(){
        let arr = [0,1,2,3,4,5,6,7,8,9,10];
        assert_eq!(ej4::cantidad_impares(arr),5);
    }
}