pub fn cantidad_de_mayores<const N:usize>(arr : [i32;N],limite : i32)->u32{
    let mut cant = 0;
    for num in arr{
        if num > limite {
            cant += 1;
        }
    }
    return cant;
}

#[cfg(test)]
mod testing_ejercicio7{
    use crate::tp2::ej7;

    #[test]
    fn caso_especial(){
        let arr1 = [0;0];
        assert_eq!(ej7::cantidad_de_mayores(arr1,0),0);
    }

    #[test]
    fn limite_cero(){
        let arr2 = [0,-7,6,-1,5,6];
        assert_eq!(ej7::cantidad_de_mayores(arr2,0),3);
    }

    #[test]
    fn limite_negativo(){
        let arr3 = [0,-7,-6,-1,-5,6];
        assert_eq!(ej7::cantidad_de_mayores(arr3,-5),3);
    }

    #[test]
    fn limite_positivo(){
        let arr4 = [0,-7,6,-1,5,6];
        assert_eq!(ej7::cantidad_de_mayores(arr4,5),2);
    }

}
